# `list`

Lists in Koto are dynamically sized contiguous arrays of values.

Like other containers in Koto, the list data is shared between copies of the
list.

## Example

```koto
x = [1, 2, "hello"]
x[1] = 99
x
# [1, 99, "hello"]

y = x
y[0] = "abc" # x and y share the same internal list data
x
# ["abc", 99, "hello"]

z = x.copy()
z[1] = -1 # z is a copy of x, so has unique internal data
x # x remains unchanged after the modificaton of z
# ["abc", 99, "hello"]
```

# Reference

## clear

`|List| -> ()`

Clears the list by removing all of its elements.

### Example

```koto
x = [1, 2, 3]
x.clear()
x
# []
```

## contains

`|List, Value| -> Bool`

Returns `true` if the list contains a value that matches the input value.

Matching is performed with the `==` equality operator.

### Example

```koto
[1, "hello", (99. -1)].contains "hello"
# true
```

## copy

`|List| -> List`

Makes a unique copy of the list data.

Note that this only copies the first level of data, so contained lists and maps
will share their data with their counterparts in the copy. To make a copy where
any contained containers are also unique, use `list.deepcopy`.

### Example

```koto
x = [1, 2, "hello"]
y = x
y[0] = "abc" # x and y share the same internal list data
x
# ["abc", 99, "hello"]

z = x.copy()
z[1] = -1 # z is a copy of x, so has unique internal data
x # x remains unchanged after the modificaton of z
# ["abc", 99, "hello"]
```

### See also

- `list.deep_copy`

## deep_copy

`|List| -> List`

Makes a unique _deep_ copy of the list data.

This makes a unique copy of the list data, and recursively makes deep copies
of any contained container data.

If only the first level of data needs to be made unique, then use `list.copy`.

### Example

```koto
x = [[1, 2], [3, [4, 5]]]
y = x.deep_copy()
y[1][1] = 99
x # a deep copy has been made, so x is unaffected by the assignment to y
# [[1, 2], [3, [4, 5]]]
```

### See also

- `list.copy`

## fill

`|List, Value| -> ()`

Fills the list with copies of the provided value.

### Example

```koto
x = [1, 2, 3]
x.fill 99
x
# [99, 99, 99]
```

## first

`|List| -> Value`

Returns the first value in the list, or `()` if the list is empty.

### Example

```koto
[99, -1, 42].first()
# 99

[].first()
# ()
```

### See also

- `list.get`
- `list.last`

## get

`|List, Number| -> Value`

Gets the Nth value in the list, or `()` if the list doesn't contain a value at
that position.

### Example

```koto
[99, -1, 42].get 1
# -1

[99, -1, 42].get 5
# ()
```

### See also

- `list.first`
- `list.last`

## insert

`|List, Number, Value| -> ()`

Inserts the value into the Nth position in the list.

An error is thrown if the position is negative or greater than the size of the
list.

### Example

```koto
[99, -1, 42].insert 2, "hello"
# [99, -1, "hello", 42]
```

### See also

- `list.remove`

## is_empty

`|List| -> Bool`

Returns `true` if the list has a size of zero, and `false` otherwise.

### Example

```koto
[].is_empty()
# true

[1, 2, 3].is_empty()
# false
```

## iter

`|List| -> Iterator`

Returns an iterator that iterates over the list's values.

Lists are iterable, so it's not necessary to call `.iter()` to get access to
iterator operations, but it can be useful sometimes to make an iterator for
later use.

### Example

```koto
x = [2, 3, 4].iter()
x.skip(1)
x.next()
# 3
```

## last

`|List| -> Value`

Returns the last value in the list, or `()` if the list is empty.

### Example

```koto
[99, -1, 42].first()
# 42

[].first()
# ()
```

### See also

- `list.first`
- `list.get`

## pop

`|List| -> Value`

Removes the last value from the list and returns it.

If the list is empty then `()` is returned.

### Example

```koto
x = [99, -1, 42]
x.pop()
# 42

x
# [99, -1]

[].pop()
# ()
```

### See also

- `list.push`

## push

`|List, Value| -> ()`

Adds the value to the end of the list.

### Example

```koto
[99, -1].push "hello"
# [99, -1, "hello"]
```

### See also

- `list.pop`

## remove

`|List, Number| -> Value`

Removes the value at the given position from the list and returns it.

Throws an error if the position isn't a valid index in the list.

### Example

```koto
[99, -1, 42].remove 1
# [99, 42]
```

### See also

- `list.insert`

## resize

## retain

## reverse

## size

## sort

## sort_copy

## swap

## to_tuple

## transform

## with_size
