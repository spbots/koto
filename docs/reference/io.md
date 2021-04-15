# `io`

A collection of utilities for working with the localo filesystem.

# Reference

- [create](#create)
- [exists](#exists)
- [open](#open)
- [read_to_string](#read_to_string)
- [remove_file](#remove_file)
- [temp_dir](#temp_dir)

## create

`|String| -> Map`

Returns an empty `File` map at the provided path.
If the file already exists it will be truncated.

### Errors

A runtime error will be thrown if the file can't be created.

### Example

```koto
f = io.create "foo.temp"
f.write_line "Hello"
f.read_to_string()
# Hello
```

## exists

`|String| -> Bool`

Returns true if a file exists at the provided path.

### Example

```koto
path = "foo.temp"
io.exists path
# false

io.create path
io.exists path
# true
```

## open

`|String| -> Map`

Opens the file at the given path, and returns a corresponding `File` map.

### Errors

An error is thrown if a file can't be opened at the given path.

### Example

```koto
f = io.open "path/to/existing.file"
# File
```

## read_to_string

`|String| -> String`

Returns a string containing the contents of the file at the given path.

### Errors

Errors are thrown:

- if the file doesn't contain valid UTF-8 data.
- if a file can't be opened at the given path.

### Example

```koto
f = io.create "foo.temp"
f.write_line "Hello!"
io.read_to_string "foo.temp"
# Hello!
```

## remove_file

`|String| -> ()`

Removes the file at the given path.

### Errors

- An error is thrown if a file can't be removed at the given path.

### Example

```koto
path "foo.temp"
io.create path
io.exists path
# true

io.remove_file path
io.exists path
# false
```

## temp_dir

`|| -> String`

Returns the path to a temporary directory.

### Note

This defers to Rust's `std::env::temp_dir`, for details see
[its documentation](https://doc.rust-lang.org/std/env/fn.temp_dir.html).
