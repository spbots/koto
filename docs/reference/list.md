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

## contains

## copy

## deep_copy

## fill

## first

## get

## insert

## is_empty

## iter

## last

## push

## remove

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
