#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum NotificationType {
    Launching = 0,
    Launched,
    Terminated,
    Hidden,
    Unhidden,
    Activated,
    Deactivated,
}

impl NotificationType {
    pub fn get_name(&self) -> &'static str {
        use NotificationType::*;
        match self {
            Launching => "NSWorkspaceWillLaunchApplicationNotification",
            Launched => "NSWorkspaceDidLaunchApplicationNotification",
            Terminated => "NSWorkspaceDidTerminateApplicationNotification",
            Hidden => "NSWorkspaceDidHideApplicationNotification",
            Unhidden => "NSWorkspaceDidUnhideApplicationNotification",
            Activated => "NSWorkspaceDidActivateApplicationNotification",
            Deactivated => "NSWorkspaceDidDeactivateApplicationNotification",
        }
    }

    pub fn get_sel(&self) -> objc::runtime::Sel {
        use NotificationType::*;
        match self {
            Launching => sel!(applicationWillLaunch:),
            Launched => sel!(applicationLaunched:),
            Terminated => sel!(applicationTerminated:),
            Hidden => sel!(applicationHidden:),
            Unhidden => sel!(applicationUnhidden:),
            Activated => sel!(applicationActivated:),
            Deactivated => sel!(applicationDeactivated:),
        }
    }
}

unsafe impl objc::Encode for NotificationType {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("i") }
    }
}

impl std::fmt::Debug for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Launching => write!(f, "launching"),
            Self::Launched => write!(f, "launched"),
            Self::Terminated => write!(f, "terminated"),
            Self::Hidden => write!(f, "hidden"),
            Self::Unhidden => write!(f, "unhidden"),
            Self::Activated => write!(f, "activated"),
            Self::Deactivated => write!(f, "deactivated"),
        }
    }
}
