use crate::{builtin_error, get_builtin_instance, single_arg_fn};
use koto_runtime::{
    value, value::type_as_string, BuiltinValue, Error, RuntimeResult, Value, ValueMap,
};
use std::{
    fmt, fs,
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

pub fn register(global: &mut ValueMap) {
    use Value::{Bool, Map, Number, Str};

    let mut io = ValueMap::new();

    single_arg_fn!(io, "exists", Str, path, {
        Ok(Bool(Path::new(path.as_ref()).exists()))
    });

    single_arg_fn!(io, "read_to_string", Str, path, {
        {
            match fs::read_to_string(Path::new(path.as_ref())) {
                Ok(result) => Ok(Str(Arc::new(result))),
                Err(e) => {
                    builtin_error!("io.read_to_string: Unable to read file '{}': {}", path, e)
                }
            }
        }
    });

    let make_file_map = || {
        fn file_fn(
            fn_name: &str,
            args: &[Value],
            mut file_op: impl FnMut(&mut File) -> RuntimeResult,
        ) -> RuntimeResult {
            get_builtin_instance!(args, "File", fn_name, File, file_ref, { file_op(file_ref) })
        }

        let mut file_map = ValueMap::new();

        file_map.add_instance_fn("path", |_, args| {
            file_fn("path", args, |file_handle| {
                Ok(Str(Arc::new(file_handle.path.to_string_lossy().to_string())))
            })
        });

        file_map.add_instance_fn("write", |_, args| {
            file_fn("write", args, |file_handle| match &args {
                [_, value] => {
                    let data = format!("{}", value);

                    match file_handle.file.write(data.as_bytes()) {
                        Ok(_) => Ok(Value::Empty),
                        Err(e) => builtin_error!("File.write: Error while writing to file: {}", e),
                    }
                }
                _ => builtin_error!("File.write: Expected single value to write as argument"),
            })
        });

        file_map.add_instance_fn("write_line", |_, args| {
            file_fn("write_line", args, |file_handle| {
                let line = match &args {
                    [_] => "\n".to_string(),
                    [_, value] => format!("{}\n", value),
                    _ => {
                        return builtin_error!(
                            "File.write_line: Expected single value as argument"
                        );
                    }
                };
                match file_handle.file.write(line.as_bytes()) {
                    Ok(_) => Ok(Value::Empty),
                    Err(e) => builtin_error!("File.write_line: Error while writing to file: {}", e),
                }
            })
        });

        file_map.add_instance_fn("read_to_string", |_, args| {
            file_fn("read_to_string", args, |file_handle| {
                match file_handle.file.seek(SeekFrom::Start(0)) {
                    Ok(_) => {
                        let mut buffer = String::new();
                        match file_handle.file.read_to_string(&mut buffer) {
                            Ok(_) => Ok(Str(Arc::new(buffer))),
                            Err(e) => builtin_error!(
                                "File.read_to_string: Error while reading data: {}",
                                e
                            ),
                        }
                    }
                    Err(e) => {
                        builtin_error!("File.read_to_string: Error while seeking in file: {}", e)
                    }
                }
            })
        });

        file_map.add_instance_fn("seek", |_, args| {
            file_fn("seek", args, |file_handle| match &args {
                [_, Number(n)] => {
                    if *n < 0.0 {
                        return builtin_error!("File.seek: Negative seek positions not allowed");
                    }
                    match file_handle.file.seek(SeekFrom::Start(*n as u64)) {
                        Ok(_) => Ok(Value::Empty),
                        Err(e) => builtin_error!("File.seek: Error while seeking in file: {}", e),
                    }
                }
                [_, unexpected] => builtin_error!(
                    "File.seek: Expected Number for seek position, found '{}'",
                    type_as_string(&unexpected)
                ),
                _ => builtin_error!("File.seek: Expected seek position as second argument"),
            })
        });

        file_map
    };

    io.add_fn("open", {
        move |_, args| match &args {
            [Str(path)] => {
                let path = Path::new(path.as_ref());
                match fs::File::open(&path) {
                    Ok(file) => {
                        let mut file_map = make_file_map();

                        file_map.set_builtin_value(File {
                            file,
                            path: path.to_path_buf(),
                            temporary: false,
                        });

                        Ok(Map(file_map))
                    }
                    Err(e) => {
                        return builtin_error!("io.open: Error while opening path: {}", e);
                    }
                }
            }
            [unexpected] => builtin_error!(
                "io.open: Expected a String as argument, found '{}'",
                type_as_string(&unexpected)
            ),
            _ => builtin_error!("io.open: Expected a String as argument"),
        }
    });

    io.add_fn("create", {
        move |_, args| match &args {
            [Str(path)] => {
                let path = Path::new(path.as_ref());
                match fs::File::create(&path) {
                    Ok(file) => {
                        let mut file_map = make_file_map();

                        file_map.set_builtin_value(File {
                            file,
                            path: path.to_path_buf(),
                            temporary: false,
                        });

                        Ok(Map(file_map))
                    }
                    Err(e) => {
                        return builtin_error!("io.create: Error while creating file: {}", e);
                    }
                }
            }
            [unexpected] => builtin_error!(
                "io.create: Expected a String as argument, found '{}'",
                type_as_string(&unexpected)
            ),
            _ => builtin_error!("io.create: Expected a String as argument"),
        }
    });

    io.add_fn("temp_path", {
        |_, _| match tempfile::NamedTempFile::new() {
            Ok(file) => match file.keep() {
                Ok((_temp_file, path)) => Ok(Str(Arc::new(path.to_string_lossy().to_string()))),
                Err(e) => builtin_error!("io.temp_file: Error while making temp path: {}", e),
            },
            Err(e) => builtin_error!("io.temp_file: Error while making temp path: {}", e),
        }
    });

    io.add_fn("temp_file", {
        move |_, _| {
            let (temp_file, path) = match tempfile::NamedTempFile::new() {
                Ok(file) => match file.keep() {
                    Ok((temp_file, path)) => (temp_file, path),
                    Err(e) => {
                        return builtin_error!(
                            "io.temp_file: Error while creating temp file: {}",
                            e
                        );
                    }
                },
                Err(e) => {
                    return builtin_error!("io.temp_file: Error while creating temp file: {}", e);
                }
            };

            let mut file_map = make_file_map();

            file_map.set_builtin_value(File {
                file: temp_file,
                path,
                temporary: true,
            });

            Ok(Map(file_map))
        }
    });

    io.add_fn("remove_file", {
        |_, args| match &args {
            [Str(path)] => {
                let path = Path::new(path.as_ref());
                match fs::remove_file(&path) {
                    Ok(_) => Ok(Value::Empty),
                    Err(e) => builtin_error!(
                        "io.remove_file: Error while removing file '{}': {}",
                        path.to_string_lossy(),
                        e
                    ),
                }
            }
            [unexpected] => builtin_error!(
                "io.remove_file: Expected a String as argument, found '{}'",
                type_as_string(&unexpected)
            ),
            _ => builtin_error!("io.remove_file: Expected a String as argument"),
        }
    });

    global.add_map("io", io);
}

#[derive(Debug)]
struct File {
    file: fs::File,
    path: PathBuf,
    temporary: bool,
}

impl Drop for File {
    fn drop(&mut self) {
        if self.temporary {
            if let Err(_) = fs::remove_file(&self.path) {};
        }
    }
}

impl BuiltinValue for File {
    fn value_type(&self) -> String {
        "File".to_string()
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File({})", self.path.to_string_lossy())
    }
}
