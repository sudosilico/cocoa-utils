use std::collections::HashMap;

use crate::prelude::*;

#[derive(CocoaType)]
pub struct Dict {
    ptr: Id,
}

impl Dict {
    pub fn get_keys(&self) -> Vec<String> {
        unsafe {
            let keys: Id = msg_send![self.ptr(), allKeys];
            let count: usize = msg_send![keys, count];

            let mut keys_vec = Vec::with_capacity(count);

            for i in 0..count {
                let key: Id = msg_send![keys, objectAtIndex: i];
                let key = NS_String::from_ptr(key).unwrap().to_string();

                keys_vec.push(key);
            }

            keys_vec
        }
    }

    pub fn into_hashmap(&self) -> HashMap<String, Id> {
        unsafe {
            let keys: Id = msg_send![self.ptr(), allKeys];
            let count: usize = msg_send![keys, count];

            let mut hashmap = HashMap::with_capacity(count);

            for i in 0..count {
                let key: Id = msg_send![keys, objectAtIndex: i];
                let value: Id = msg_send![self.ptr(), objectForKey: key];
                let key = NS_String::from_ptr(key).unwrap().to_string();

                hashmap.insert(key, value);
            }

            hashmap
        }
    }

    pub fn get_id(&self, key: &str) -> Option<Id> {
        unsafe {
            let key = NS_String::from(key).ptr();
            let value: Id = msg_send![self.ptr(), objectForKey: key];

            if value.is_null() {
                None
            } else {
                Some(value)
            }
        }
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        unsafe {
            let key = NS_String::from(key).ptr();
            let value: Id = msg_send![self.ptr(), objectForKey: key];

            if value.is_null() {
                None
            } else {
                Some(NS_String::from_ptr(value).unwrap().to_string())
            }
        }
    }

    pub fn get_value<T: 'static>(&self, key: &str) -> T {
        unsafe {
            let key = NS_String::from(key).ptr();
            let value: T = msg_send![self.ptr(), objectForKey: key];

            value
        }
    }

    pub fn map<T>(self, callback: impl Fn(Id, Id) -> T) -> Vec<T> {
        unsafe {
            let keys: Id = msg_send![self.ptr, allKeys];
            let count: usize = msg_send![keys, count];

            let mut vec: Vec<T> = vec![];

            for i in 0..count {
                let key: Id = msg_send![keys, objectAtIndex: i];
                let value: Id = msg_send![self.ptr, objectForKey: key];

                let result = callback(key, value);
                vec.push(result);
            }

            vec
        }
    }
}
