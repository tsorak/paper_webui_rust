use std::fmt::Display;

use instance::ServerExecutable;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    Unexpected,
    //
    ServerExecutable(ServerExecutable),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unexpected => {
                //
                write!(f, "Unexpected library error")
            }
            Self::ServerExecutable(e) => {
                //
                if let Some(p) = &e.path {
                    write!(f, "{}: '{}'", e.message, p.to_string_lossy())
                } else {
                    write!(f, "{}", e.message)
                }
            }
        }
    }
}

pub mod instance {
    use std::path::PathBuf;

    #[derive(Debug)]
    pub struct ServerExecutable {
        pub path: Option<PathBuf>,
        pub message: String,
    }

    impl ServerExecutable {
        pub fn new(path: PathBuf, message: impl AsRef<str>) -> crate::core::Error {
            crate::core::Error::ServerExecutable(Self {
                path: Some(path),
                message: message.as_ref().to_string(),
            })
        }

        pub fn msg(message: impl AsRef<str>) -> crate::core::Error {
            crate::core::Error::ServerExecutable(Self {
                path: None,
                message: message.as_ref().to_string(),
            })
        }
    }
}
