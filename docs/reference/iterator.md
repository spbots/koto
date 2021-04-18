# `iterator`

Iterators in Koto provide access to sequences of data, yielding values via
`.next()`, until the end of the sequence is reached, and the empty value `()` is
returned.

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

The contents of the `iterator` module are implicitly available to all
`Iterable`s.

For example:

```koto
# Starting with a List
[1, 2, 3]
  # Calling iterator.each with the list as the implicit first argument
  .each |x| x * 2
  # Calling iterator.to_list with the Iterator from iterator.each
  .to_list()
# [2, 4, 6]
```

## Loops

`for`, `while`, and `until` loops take an `Iterable` and then provide the
yielded values for each iteration of the loop.
