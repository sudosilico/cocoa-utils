#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::ParseStream;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Item, Result};

#[proc_macro_derive(CocoaType)]
pub fn derive_cocoatype(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let type_name = &ast.ident;

    let result = quote! {
        impl CocoaType for #type_name {
            unsafe fn from_ptr(ptr: Id) -> Option<Self> {
                if ptr.is_null() {
                    None
                } else {
                    Some(Self { ptr })
                }
            }

            unsafe fn ptr(&self) -> Id {
                self.ptr
            }

            fn class_name() -> &'static str {
                stringify!(#type_name)
            }
        }
    };

    result.into()
}

struct CocoaInstancePropertyMeta {
    selector: syn::Ident,
}

impl syn::parse::Parse for CocoaInstancePropertyMeta {
    fn parse(input: ParseStream) -> Result<Self> {
        let selector = input.parse::<syn::Ident>()?;
        Ok(CocoaInstancePropertyMeta {
            selector: proc_macro2::Ident::new(&selector.to_string(), selector.span()),
        })
    }
}

struct CocoaTypePropertyMeta {
    class_name: syn::Ident,
    selector: syn::Ident,
}

impl syn::parse::Parse for CocoaTypePropertyMeta {
    fn parse(input: ParseStream) -> Result<Self> {
        let class_name = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![,]>()?;
        let selector = input.parse::<syn::Ident>()?;
        Ok(CocoaTypePropertyMeta {
            class_name: proc_macro2::Ident::new(&class_name.to_string(), class_name.span()),
            selector: proc_macro2::Ident::new(&selector.to_string(), selector.span()),
        })
    }
}

#[proc_macro_attribute]
pub fn cocoa_instance_property(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut item: syn::Item = syn::parse(input).expect("failed to parse input");

    match item {
        Item::Fn(ref mut fn_item) => {
            let meta = parse_macro_input!(metadata as CocoaInstancePropertyMeta);
            append_instance_property(fn_item, &meta);
        }
        _ => {
            item.span()
                .unwrap()
                .error("cocoa_instance_property can only be applied to functions")
                .emit();
        }
    }

    let output = quote! { #item };
    output.into()
}

#[proc_macro_attribute]
pub fn cocoa_type_property(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut item: syn::Item = syn::parse(input).expect("failed to parse input");

    match item {
        Item::Fn(ref mut fn_item) => {
            let meta = parse_macro_input!(metadata as CocoaTypePropertyMeta);
            append_type_property(fn_item, &meta);
        }
        _ => {
            item.span()
                .unwrap()
                .error("cocoa_instance_property can only be applied to functions")
                .emit();
        }
    }

    let output = quote! { #item };
    output.into()
}

fn get_return_type(return_type: &syn::ReturnType) -> &syn::Type {
    match return_type {
        syn::ReturnType::Default => panic!("return type must be specified"),
        syn::ReturnType::Type(_, ref ty) => ty,
    }
}

fn is_std_string(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(ref path) => {
            let path = &path.path;
            path.segments.len() == 1 && path.segments[0].ident == "String"
        }
        _ => false,
    }
}

fn is_id(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(ref path) => {
            let path = &path.path;
            path.segments.len() == 1 && path.segments[0].ident == "Id"
        }
        _ => false,
    }
}

fn is_value_type(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(ref path) => {
            let value_types = vec![
                "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "f32", "f64", "bool",
                "isize", "usize",
            ];

            let path = &path.path;

            path.segments.len() == 1
                && value_types.contains(&path.segments[0].ident.to_string().as_str())
        }
        _ => false,
    }
}

fn append_instance_property(fn_item: &mut syn::ItemFn, meta: &CocoaInstancePropertyMeta) {
    let return_type: &syn::ReturnType = &fn_item.sig.output.clone();
    let mut return_type = get_return_type(return_type);
    let selector = &meta.selector;

    // Option<T>
    let optional = if let Some(option_inner) = get_option_type(return_type) {
        return_type = option_inner;
        true
    } else {
        false
    };

    if is_std_string(return_type) {
        append_string_instance_property(fn_item, selector, optional);
    } else if is_id(return_type) {
        append_id_instance_property(fn_item, selector, optional);
    } else if is_value_type(return_type) {
        if optional {
            panic!("value types cannot be optional");
        } else {
            append_value_instance_property(fn_item, return_type, selector);
        }
    } else {
        append_cocoatype_instance_property(fn_item, return_type, selector, optional);
    }
}

fn append_string_instance_property(
    fn_item: &mut syn::ItemFn,
    selector: &proc_macro2::Ident,
    optional: bool,
) {
    let body = if optional {
        quote! {{
            use ::cocoa::foundation::NSAutoreleasePool;

            let pool = NSAutoreleasePool::new(cocoa::base::nil);
            let ptr: Id = self.ptr();
            let ptr: Id = msg_send![ptr, #selector];

            let result = if ptr.is_null() {
                None
            } else {
                let string: *const ::std::ffi::c_char = msg_send![ptr, UTF8String];
                let string: String = ::std::ffi::CStr::from_ptr(string).to_string_lossy().into_owned().to_string();

                Some(string)
            };

            pool.drain();
            result
        }}
    } else {
        quote! {{
            use ::cocoa::foundation::NSAutoreleasePool;

            let pool = NSAutoreleasePool::new(cocoa::base::nil);
            let ptr: Id = self.ptr();
            let ptr: Id = msg_send![ptr, #selector];

            if ptr.is_null() {
                panic!("Property getter returned a pull ptr. You may need to wrap the return type in Option<T>.");
            }

            let string: *const ::std::ffi::c_char = msg_send![ptr, UTF8String];
            let string: String = ::std::ffi::CStr::from_ptr(string).to_string_lossy().into_owned().to_string();

            pool.drain();
            string
        }}
    };

    fn_item.block.stmts.push(syn::parse2(body).unwrap());
}

fn append_value_instance_property(
    fn_item: &mut syn::ItemFn,
    value_type: &syn::Type,
    selector: &proc_macro2::Ident,
) {
    let return_statement = {
        quote! {{
            let ptr: Id = self.ptr();
            let result: #value_type = msg_send![ptr, #selector];

            return result;
        }}
    };

    fn_item
        .block
        .stmts
        .push(syn::parse2(return_statement).unwrap());
}

fn append_id_instance_property(
    fn_item: &mut syn::ItemFn,
    selector: &proc_macro2::Ident,
    optional: bool,
) {
    let return_statement = if optional {
        quote! {{
            let ptr: Id = self.ptr();
            let ptr: Id = msg_send![ptr, #selector];

            let result = if ptr.is_null() {
                None
            } else {
                Some(ptr)
            };

            return result;
        }}
    } else {
        quote! {{
            let ptr: Id = self.ptr();
            let ptr: Id = msg_send![ptr, #selector];

            if ptr.is_null() {
                panic!("Property getter returned a null ptr. You may need to wrap the return type in Option<T>.");
            }

            return ptr;
        }}
    };
    fn_item
        .block
        .stmts
        .push(syn::parse2(return_statement).unwrap());
}

fn append_cocoatype_instance_property(
    fn_item: &mut syn::ItemFn,
    return_type: &syn::Type,
    selector: &proc_macro2::Ident,
    optional: bool,
) {
    let return_statement = if optional {
        quote! {{
            let ptr: Id = self.ptr();
            let ptr: Id = msg_send![ptr, #selector];

            let result = #return_type::from_ptr(ptr);
            return result;
        }}
    } else {
        quote! {{
            let ptr: Id = self.ptr();
            let ptr: Id = msg_send![ptr, #selector];

            if ptr.is_null() {
                panic!("Property getter returned a null ptr. You may need to wrap the return type in Option<T>.");
            }

            return #return_type::from_ptr(ptr).unwrap();
        }}
    };
    fn_item
        .block
        .stmts
        .push(syn::parse2(return_statement).unwrap());
}

fn get_option_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != "Option" {
            return None;
        }

        if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None;
            }

            if let syn::GenericArgument::Type(ref ty) = inner_ty.args[0] {
                return Some(ty);
            }
        }
    }
    None
}

fn append_type_property(fn_item: &mut syn::ItemFn, meta: &CocoaTypePropertyMeta) {
    let return_type: &syn::ReturnType = &fn_item.sig.output.clone();
    let mut return_type = get_return_type(return_type);
    let selector = &meta.selector;

    // Option<T>
    let optional = if let Some(option_inner) = get_option_type(return_type) {
        return_type = option_inner;
        true
    } else {
        false
    };

    let class_name = &meta.class_name;

    if is_std_string(return_type) {
        append_string_type_property(fn_item, class_name, selector, optional);
    } else if is_id(return_type) {
        append_id_type_property(fn_item, class_name, selector, optional);
    } else {
        append_cocoatype_type_property(fn_item, return_type, class_name, selector, optional);
    }
}

fn append_string_type_property(
    fn_item: &mut syn::ItemFn,
    class_name: &proc_macro2::Ident,
    selector: &proc_macro2::Ident,
    optional: bool,
) {
    let body = if optional {
        quote! {{
            use ::cocoa::foundation::NSAutoreleasePool;

            let pool = NSAutoreleasePool::new(cocoa::base::nil);
            let ptr: Id = msg_send![class!(#class_name), #selector];

            let result = if ptr.is_null() {
                None
            } else {
                let string: *const ::std::ffi::c_char = msg_send![ptr, UTF8String];
                let string: String = ::std::ffi::CStr::from_ptr(string).to_string_lossy().into_owned().to_string();

                Some(string)
            };

            pool.drain();
            result
        }}
    } else {
        quote! {{
            use ::cocoa::foundation::NSAutoreleasePool;

            let pool = NSAutoreleasePool::new(cocoa::base::nil);
            let ptr: Id = msg_send![class!(#class_name), #selector];

            if ptr.is_null() {
                panic!("Property getter returned a pull ptr. You may need to wrap the return type in Option<T>.");
            }

            let string: *const ::std::ffi::c_char = msg_send![ptr, UTF8String];
            let string: String = ::std::ffi::CStr::from_ptr(string).to_string_lossy().into_owned().to_string();

            pool.drain();
            string
        }}
    };

    fn_item.block.stmts.push(syn::parse2(body).unwrap());
}

fn append_id_type_property(
    fn_item: &mut syn::ItemFn,
    class_name: &proc_macro2::Ident,
    selector: &proc_macro2::Ident,
    optional: bool,
) {
    let return_statement = if optional {
        quote! {{
            let ptr: Id = msg_send![class!(#class_name), #selector];

            let result = if ptr.is_null() {
                None
            } else {
                Some(ptr)
            };

            return result;
        }}
    } else {
        quote! {{
            let ptr: Id = msg_send![class!(#class_name), #selector];

            if ptr.is_null() {
                panic!("Property getter returned a null ptr. You may need to wrap the return type in Option<T>.");
            }

            return ptr;
        }}
    };
    fn_item
        .block
        .stmts
        .push(syn::parse2(return_statement).unwrap());
}

fn append_cocoatype_type_property(
    fn_item: &mut syn::ItemFn,
    return_type: &syn::Type,
    class_name: &proc_macro2::Ident,
    selector: &proc_macro2::Ident,
    optional: bool,
) {
    let return_statement = if optional {
        quote! {{
            let ptr: Id = msg_send![class!(#class_name), #selector];

            let result = #return_type::from_ptr(ptr);
            return result;
        }}
    } else {
        quote! {{
            let ptr: Id = msg_send![class!(#class_name), #selector];

            if ptr.is_null() {
                panic!("Property getter returned a null ptr. You may need to wrap the return type in Option<T>.");
            }

            return #return_type::from_ptr(ptr).unwrap();
        }}
    };
    fn_item
        .block
        .stmts
        .push(syn::parse2(return_statement).unwrap());
}
