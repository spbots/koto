# `iterator`

Iterators in Koto provide access to sequences of data, yielding values via
`.next()`, until the end of the sequence is reached, and the empty value `()`
is returned.

## Iterable Values

Values that can produce iterable sequences are referred to as `Iterable`.
`Iterables` include:

- Iterators (naturally!)
- Lists
- Maps
  - The map's key/value pairs are provided as a Tuple.
- Ranges
- Strings
- Tuples

The contents of the `iterator` module are made available to all `Iterable`s.

### Example

```koto
# Starting with a List
[1, 2, 3]
  # Calling iterator.each with the List as the implicit first argument
  .each |x| x * 2
  # Calling iterator.to_list with the Iterator resulting from iterator.each
  .to_list()
# [2, 4, 6]
```

## Loops

`for`, `while`, and `until` loops take any `Iterable` and then provide its
output values for each iteration of the loop.

```koto
for x in (2, 3, 4).each |n| n * 2
  "-> {}".print x
# -> 4
# -> 6
# -> 8
```

# Reference

## all

`|Iterable, Function(|Value| -> Bool)| -> Bool`

Checks the Iterable's values against a test Function.
The Function should return `true` or `false`, and then `all` returns `true`
if all values pass the test.

`all` stops running as soon as it finds a failing test, and then `false` is
returned.

### Example

```koto
(1..9).all |x| x > 0
# true

("", "", "foo").all string.is_empty
# false

[10, 20, 30]
  .each |x| x / 10
  .all |x| x < 10
# true
```

## all

`|Iterable, |Value| -> Bool| -> Bool`

Checks the Iterable's values against a test Function.
The Function should return `true` or `false`, and then `all` returns `true`
if all values pass the test.

`all` stops running as soon as it finds a failing test, and then `false` is
returned.

### Example

```koto
(1..9).all |x| x > 0
# true

("", "", "foo").all string.is_empty
# false

[10, 20, 30]
  .each |x| x / 10
  .all |x| x < 10
# true
```

## any

`|Iterable, |Value| -> Bool| -> Bool`

Checks the Iterable's values against a test Function.
The Function should return `true` or `false`, and then `any` returns `true`
if any of the values pass the test.

`any` stops running as soon as it finds a passing test.

### Example

```koto
(1..9).any |x| x == 5
# true

("", "", "foo").any string.is_empty
# true

[10, 20, 30]
  .each |x| x / 10
  .any |x| x == 2
# true
```

## chain

`|Iterable, Iterable| -> Iterator`

`chain` returns an iterator that iterates over the output of the first iterator,
followed by the output of the second iterator.

### Example

```koto
(1, 2).chain(3..=5).to_tuple()
# (1, 2, 3, 4, 5)
```

## consume

`|Iterable| -> ()`

Consumes the output of the iterator. This is useful when the side-effects of
the iterator chain are important, but not so much the output value.

## count

`|Iterable| -> Number`

Counts the number of items yielded from the iterator.

### Example

```koto
(5..=15).count()
# 10

(0..100)
  .keep |x| x % 2 == 0
  .count()
# 50
```

## each

`|Iterable, |Value| -> Value| -> Iterator`

Takes an Iterable and a Function, and returns a new iterator that provides the
result of calling the function with each value in the iterable.

### Example

```koto
(2, 3, 4)
  .each |x| x * 2
  .to_list()
# [4, 6, 8]
```

## enumerate

`|Iterable| -> Iterator`

Returns an iterator that provides each value along with an associated index.

### Example

```koto
("a", "b", "c").enumerate().to_list()
# [(0, "a"), (1, "b"), (2, "c")]
```

## fold

`|Iterable, Value, |Value, Value| -> Value| -> Value`

Returns the result of 'folding' the iterator's values into an accumulator
function.

The function takes the accumulated value and the next iterator value,
and then returns the result of folding the value into the accumulator.

The first argument is an initial accumulated value that gets passed to the
function along with the first value from the iterator.

The result is then the final accumulated value.

This operation is also known in other languages as `reduce`, `accumulate`,
`inject`, `fold left` (along with other names), with `fold` chosen here for

### Example

```koto
("a", "b", "c").fold "", |result, x| result += x + "-"
# a-b-c-
```

### See Also

- `iterator.product`
- `iterator.sum`

## keep

`|Iterable, |Value| -> Bool| -> Iterator`

Returns an iterator that keeps only the values that pass a test function.

The function is called for each value in the iterator, and returns either `true`
if the value should be kept in the iterator output, or `false` if it should be
discarded.

### Example

```koto
(0..10).keep(|x| x % 2 == 0).to_tuple()
# (0, 2, 4, 6, 8)
```

## max

`|Iterable| -> Value`

Returns the maximum value found in the iterable.

A `<` 'less than' comparison is performed between each value and the maximum
found so far, until all values in the iterator have been compared.

### Example

```koto
(8, -3, 99, -1).max()
# 99
```

### See Also

- `iterator.min`
- `iterator.min_max`

## min

`|Iterable| -> Value`

Returns the minimum value found in the iterable.

A `<` 'less than' comparison is performed between each value and the minimum
found so far, until all values in the iterator have been compared.

### Example

```koto
(8, -3, 99, -1).min()
# -3
```

### See Also

- `iterator.max`
- `iterator.min_max`

## min_max

`|Iterable| -> (Value, Value)`

Returns the minimum and maximum values found in the iterable.

A `<` 'less than' comparison is performed between each value and both the
minimum and maximum found so far, until all values in the iterator have been
compared.

### Example

```koto
(8, -3, 99, -1).min_max()
# (-3, 99)
```

### See Also

- `iterator.max`
- `iterator.min`

## next

`|Iterator| -> Value`

Returns the next value from the iterator.

### Example

```koto
x = (1, 2).iter()
x.next()
# 1
x.next()
# 2
x.next()
# ()
```

## position

`|Iterable, |Value| -> Bool| -> Value`

Returns the position of the first value in the iterable that passes the test
function.

The function is called for each value in the iterator, and returns either `true`
if the value is a match, or `false` if it's not.

The first matching value will cause iteration to stop, and the number of
steps taken to reach the matched value is returned as the result.

If no match is found then `()` is returned.

### Example

```koto
(10..20).position |x| x == 15
# 5

(10..20).position |x| x == 99
# ()
```

## product

`|Iterable| -> Value`

Returns the result of multiplying each value in the iterable together.

### Example

```koto
(2, 3, 4).product()
# 24
```

### See also

- `iterator.fold`
- `iterator.sum`

## skip

`|Iterable, Number| -> Iterator`

Skips over a number of steps in the iterator.

### Example

```koto
(100..200).skip(50).next()
# 150
```

### See also

- `iterator.take`

## sum

`|Iterable| -> Value`

Returns the result of adding each value in the iterable together.

### Example

```koto
(2, 3, 4).sum()
# 9
```

### See also

- `iterator.fold`
- `iterator.product`

## take

`|Iterable, Number| -> Iterator`

Provides an iterator that consumes a number of values from the input before
finishing.

### Example

```koto
(100..200).take(3).to_tuple()
# (100, 101, 102)
```

### See also

- `iterator.skip`

## to_list

`|Iterable| -> List`

Consumes all values coming from the iterator and places them in a list.

### Example

```koto
("a", 42, (-1, -2)).to_list()
# ["a", 42, (-1, -2)]
```

### See also

- `iterator.to_map`
- `iterator.to_tuple`



## to_map

`|Iterable| -> Map`

Consumes all values coming from the iterator and places them in a map.

If a value is a tuple, then the first element in the tuple will be inserted as
the key for the map entry, and the second element will be inserted as the value.

If the value is anything other than a tuple, then it will be inserted as the map
key, with `()` as the entry's value.

### Example

```koto
("a", "b", "c").to_map()
# {"a": (), "b": (), "c": ()}

("a", "bbb", "cc")
  .each |x| x, x.size()
  .to_map()
# {"a": 1, "bbb": 3, "cc": 2}
```

### See also

- `iterator.to_list`
- `iterator.to_tuple`

## to_tuple

`|Iterable| -> Tuple`

Consumes all values coming from the iterator and places them in a tuple.

### Example

```koto
("a", 42, (-1, -2)).to_list()
# ["a", 42, (-1, -2)]
```

### See also

- `iterator.to_list`
- `iterator.to_map`


## zip

`|Iterable, Iterable| -> Iterator`

Combines the values in two iterables into an iterator that provides
corresponding pairs of values, one at a time from each input iterable.

### Example

```koto
(1, 2, 3).zip(("a", "b", "c")).to_list()
# [(1, "a"), (2, "b"), (3, "c")]
```

