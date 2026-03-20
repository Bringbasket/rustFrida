pub mod config;
pub mod protocol;

pub use config::{BootstrapConfig, ControllerConfig, InjectionMode};
pub use protocol::{
    read_frame, write_frame, AgentEvent, FrameKind, Hello, FRAME_KIND_CMD, FRAME_KIND_COMPLETE,
    FRAME_KIND_EVAL_ERR, FRAME_KIND_EVAL_OK, FRAME_KIND_HELLO, FRAME_KIND_LOG,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Protocol(String),
    InvalidArgument(String),
    State(String),
    Unsupported(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "io error: {err}"),
            Error::Protocol(msg) => write!(f, "protocol error: {msg}"),
            Error::InvalidArgument(msg) => write!(f, "invalid argument: {msg}"),
            Error::State(msg) => write!(f, "invalid state: {msg}"),
            Error::Unsupported(msg) => write!(f, "unsupported: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}
