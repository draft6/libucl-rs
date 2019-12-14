use libucl_bind::ucl_error_t;

use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub enum UclErrorType {
    Ok,
    Syntax,
    Io,
    State,
    Nested,
    Macro,
    Internal,
    SSL,
    Other
}

impl UclErrorType {
    pub fn from_code(num: i32, desc: String) -> UclError {
        match num {
            _ if num == ucl_error_t::UCL_EOK       as i32 => UclError{code: UclErrorType::Ok, desc: desc},
            _ if num == ucl_error_t::UCL_ESYNTAX   as i32 => UclError{code: UclErrorType::Syntax, desc: desc},
            _ if num == ucl_error_t::UCL_EIO       as i32 => UclError{code: UclErrorType::Io, desc: desc},
            _ if num == ucl_error_t::UCL_ESTATE    as i32 => UclError{code: UclErrorType::State, desc: desc},
            _ if num == ucl_error_t::UCL_ENESTED   as i32 => UclError{code: UclErrorType::Nested, desc: desc},
            _ if num == ucl_error_t::UCL_EMACRO    as i32 => UclError{code: UclErrorType::Macro, desc: desc},
            _ if num == ucl_error_t::UCL_EINTERNAL as i32 => UclError{code: UclErrorType::Internal, desc: desc},
            _ if num == ucl_error_t::UCL_ESSL      as i32 => UclError{code: UclErrorType::SSL, desc: desc},
            _ => UclError{code: UclErrorType::Other, desc: desc}
        }
    }
}

#[derive(Clone, Debug)]
pub struct UclError {
    code: UclErrorType,
    desc: String
}

impl fmt::Display for UclError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl Error for UclError {
    fn description(&self) -> &str {
        self.desc.as_ref()
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}
