# Solution Day 1 

## Tasks

(This is a technical summary of the tasks without the nice story behind it, so go the the website and look at it yourself for more details)

### Task 1

Basically load a CSV-file with two columns, sort the columns individually and sum up the resulting pairs.

### Task 2

Slight variation: instead of summing up the pairs, use the first column as a search key to count the occurencences in the second column and
then multiply each key with it's number of occurences, summing up the products.

Note: The "key" in the first column can occur multiple times and isn't therefore really a key at all.

## Solutions

Both solutions will fail if the format even slightly deviated from the one given in the example. Only numbers, no characters etc. Basically: production
code should get rid of the `unwrap()`s.

### Task 1

Straightforward. Will fail horribly at runtime, if the second column has fewer values, that the first. Also: Haskell gurus and other FP cracks
will probably haunt me for the use of `for .. in ..` and mutable variables instead of `fold()` (which Rust would have. Couldn't be bothered though).

Also I learned, that Rust's `Vec<T>` has an in-place sort like Javascript. Makes sense from the perspective of a memory-efficient language. Was
surprising to me anyway.

### Task 2

Arguably more robust than Task 1 because it doesn't assume anything about the size of column 2. Will fail at runtime, if the the number of occurences
of a key goes beyond `i32` (2^31, so well beyond the realm of the Advent of Code tests)