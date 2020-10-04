mod core;
mod external;
mod frame;
mod loader;
pub mod value;
mod value_iterator;
mod value_list;
mod value_map;
mod vm;

use {
    koto_bytecode::Chunk,
    std::{fmt, sync::Arc},
};

pub use {
    external::{ExternalFunction, ExternalValue},
    loader::{Loader, LoaderError},
    value::{
        make_external_value, type_as_string, value_is_immutable, RuntimeFunction, Value, ValueRef,
    },
    value_iterator::IntRange,
    value_list::{ValueList, ValueVec},
    value_map::{ValueHashMap, ValueMap, ValueMapKey},
    vm::Vm,
};

#[derive(Clone, Debug)]
pub enum Error {
    VmError {
        message: String,
        chunk: Arc<Chunk>,
        instruction: usize,
    },
    LoaderError(loader::LoaderError),
    ErrorWithoutLocation {
        message: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::VmError { message, .. } => f.write_str(message),
            Error::LoaderError(error) => f.write_str(&error.message),
            Error::ErrorWithoutLocation { message } => f.write_str(message),
        }
    }
}

pub type RuntimeResult = Result<Value, Error>;

#[macro_export]
macro_rules! make_vm_error {
    ($chunk:expr, $ip:expr, $message:expr) => {{
        let error = $crate::Error::VmError {
            message: $message,
            chunk: $chunk,
            instruction: $ip,
        };
        #[cfg(panic_on_runtime_error)]
        {
            panic!();
        }
        error
    }};
}

#[macro_export]
macro_rules! vm_error {
    ($chunk:expr, $ip:expr, $error:expr) => {
        Err($crate::make_vm_error!($chunk, $ip, String::from($error)))
    };
    ($chunk:expr, $ip:expr, $error:expr, $($y:expr),+ $(,)?) => {
        Err($crate::make_vm_error!($chunk, $ip, format!($error, $($y),+)))
    };
}

#[macro_export]
macro_rules! make_external_error {
    ($message:expr) => {{
        let error = $crate::Error::ErrorWithoutLocation { message: $message };
        #[cfg(panic_on_runtime_error)]
        {
            panic!();
        }
        error
    }};
}

#[macro_export]
macro_rules! external_error {
    ($error:expr) => {
        Err($crate::make_external_error!(String::from($error)))
    };
    ($error:expr, $($y:expr),+ $(,)?) => {
        Err($crate::make_external_error!(format!($error, $($y),+)))
    };
}
