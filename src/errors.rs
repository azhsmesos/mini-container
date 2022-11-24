use std::fmt;
use std::fmt::Formatter;
use std::process::exit;

#[derive(Debug)]
pub enum Errcode {
    ArgumentInvalid(&'static str),
    ContainerError(u8),
    NotSupported(u8),
    SocketError(u8),
    ChildProcessError(u8),
    HostnameError(u8),
    RngError,
    MountsError(u8),
    NamespaceError(u8),
}

impl Errcode {
    pub fn get_ret_code(&self) -> i32 {
        1
    }
}

#[allow(unreachable_patterns)]
impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Errcode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid: {}", element),
            _ => write!(f, "{:?}", self),
        }
    }
}

// Get the result from a function, and exit the process with the correct error code
pub fn exit_with_ret_code(res: Result<(), Errcode>) {
    match res {
        Ok(_) => {
            log::debug!("Exit without any error, returning 0");
            exit(0);
        }
        Err(e) => {
            let code = e.get_ret_code();
            log::error!("Error on exit:\n\t{}\n\tReturning {}", e, code);
            exit(code);
        }
    }
}
