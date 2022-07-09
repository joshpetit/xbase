use process_stream::ProcessItem;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use typescript_definitions::TypeScriptify;

/// Representation of Messages that clients needs to process
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, TypeScriptify)]
#[serde(tag = "type", content = "args")]
pub enum Message {
    /// Notify use with a message
    Notify { msg: String, level: MessageLevel },
    /// Log a message
    Log { msg: String, level: MessageLevel },
    /// Execute an task
    Execute(Task),
}

/// Statusline state
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, TypeScriptify)]
pub enum StatuslineState {
    /// Clear statusline
    Clear,
    /// Last task failed
    Failure,
    /// A Request is being processed.
    Processing,
    /// that is currently running.
    Running,
    /// Last task was successful
    Success,
    /// Something is being watched.
    Watching,
}

/// Message Level
#[derive(
    Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, TypeScriptify,
)]
#[repr(u8)]
pub enum MessageLevel {
    /// Trace Message
    Trace = 0,
    /// Debug Message
    Debug = 1,
    /// Info Message
    Info = 2,
    /// Warn Message
    Warn = 3,
    /// Error Message
    Error = 4,
    /// Success Message
    Success = 5,
}

impl std::fmt::Display for StatuslineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            StatuslineState::Success => "success",
            StatuslineState::Failure => "failure",
            StatuslineState::Processing => "processing",
            StatuslineState::Watching => "watching",
            StatuslineState::Running => "running",
            StatuslineState::Clear => "",
        };
        write!(f, "{value}")
    }
}

/// Tasks that the clients should execute
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, TypeScriptify)]
#[serde(tag = "task")]
pub enum Task {
    OpenLogger,
    ReloadLspServer,
    UpdateStatusline { value: StatuslineState },
}

impl From<ProcessItem> for Message {
    fn from(item: ProcessItem) -> Self {
        let is_success = item.is_success();
        match item {
            ProcessItem::Output(value) => {
                if value.to_lowercase().contains("error") {
                    Self::Log {
                        msg: value,
                        level: MessageLevel::Error,
                    }
                } else if value.to_lowercase().contains("warn") {
                    Self::Log {
                        msg: value,
                        level: MessageLevel::Warn,
                    }
                } else {
                    Self::Log {
                        msg: if value == "Resolving Packages" {
                            Default::default()
                        } else {
                            value
                        },
                        level: MessageLevel::Info,
                    }
                }
            }
            ProcessItem::Error(value) => Self::Log {
                msg: value,
                level: MessageLevel::Error,
            },
            ProcessItem::Exit(code) => {
                if is_success.unwrap() {
                    Self::Log {
                        msg: Default::default(),
                        level: MessageLevel::Info,
                    }
                } else {
                    Self::Log {
                        msg: format!("[Error] {code} code"),
                        level: MessageLevel::Error,
                    }
                }
            }
        }
    }
}

impl From<String> for Message {
    fn from(value: String) -> Self {
        Self::Notify {
            msg: value,
            level: MessageLevel::Info,
        }
    }
}

impl From<&str> for Message {
    fn from(value: &str) -> Self {
        Self::Notify {
            msg: value.to_string(),
            level: MessageLevel::Info,
        }
    }
}

impl Message {
    pub fn notify_error<S: AsRef<str>>(value: S) -> Self {
        Self::Notify {
            msg: value.as_ref().to_string(),
            level: MessageLevel::Error,
        }
    }

    pub fn notify_warn<S: AsRef<str>>(value: S) -> Self {
        Self::Notify {
            msg: value.as_ref().to_string(),
            level: MessageLevel::Warn,
        }
    }

    pub fn notify_trace<S: AsRef<str>>(value: S) -> Self {
        Self::Notify {
            msg: value.as_ref().to_string(),
            level: MessageLevel::Trace,
        }
    }

    pub fn notify_debug<S: AsRef<str>>(value: S) -> Self {
        Self::Notify {
            msg: value.as_ref().to_string(),
            level: MessageLevel::Debug,
        }
    }

    pub fn log_error<S: AsRef<str>>(value: S) -> Self {
        Self::Log {
            msg: value.as_ref().to_string(),
            level: MessageLevel::Error,
        }
    }

    pub fn log_info<S: AsRef<str>>(value: S) -> Self {
        Self::Log {
            msg: value.as_ref().to_string(),
            level: MessageLevel::Info,
        }
    }

    pub fn log_warn<S: AsRef<str>>(value: S) -> Self {
        Self::Log {
            msg: value.as_ref().to_string(),
            level: MessageLevel::Warn,
        }
    }

    pub fn log_trace<S: AsRef<str>>(value: S) -> Self {
        Self::Log {
            msg: value.as_ref().to_string(),
            level: MessageLevel::Trace,
        }
    }

    pub fn log_debug<S: AsRef<str>>(value: S) -> Self {
        Self::Log {
            msg: value.as_ref().to_string(),
            level: MessageLevel::Debug,
        }
    }
}