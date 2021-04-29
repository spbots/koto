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
