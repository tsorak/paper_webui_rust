use std::fmt::Display;

use instance::ServerExecutable;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    Unexpected,
    //
    ServerExecutable(ServerExecutable),
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        dbg!(value);
        Error::Unexpected
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unexpected => {
                //
                write!(f, "Unexpected paper_webui::core error")
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
        pub fn box_new(path: PathBuf, message: impl AsRef<str>) -> crate::Error {
            crate::Error::ServerExecutable(Self {
                path: Some(path),
                message: message.as_ref().to_string(),
            })
        }

        pub fn box_msg(message: impl AsRef<str>) -> crate::Error {
            crate::Error::ServerExecutable(Self {
                path: None,
                message: message.as_ref().to_string(),
            })
        }
    }
}
