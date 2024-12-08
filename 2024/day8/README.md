# Solution Day 8

A bit of geometry for a change. The task was quite ambiguous, I felt. The text says:

>  In particular, an antinode occurs at any point that is perfectly in line with two antennas of the same frequency - but only when one of the antennas is twice as far away as the other

and then goes on to give exambles, where the antinodes lie outside the two antennas, like this:

```
..#....a....a....#....
``` 

Either I'm not getting the exact meaning of "antinode" (English is not my native language) or there's another set of antinodes _between_ the two antennas:

```
..#....a.##.a....#....
```

Those should also fulfill the requirement of one antenna being twice as far away as the other.
Anyway, going with the examples, this is basically just calculating an equation of the form
`p = mn + b`, with `p`, and `b` being points in 2D space, `m = a - b` (the vector between
antennas `a` and `b`), and `n` being in the natural numbers **N**.

Implementing this as infrastructure (along with loading and parsing the data), makes the actual tasks rather simple.

## Task 1

The main point in this task is to only include points of order 1 formed by 2 different antennas, hence
why the inner nested loop in `get_antinodes` has to always start one index _above_ the current outer loop,
to never pair an antenna with itself (I got this wrong in the first try). Those nested loops will form all possible pairs
of the antennas:

```
[1,2,3] => [[1,2], [1,3], [2,3]]
```

## Task 2

The change to the code are actually miniscule: antennas can now interfere with themselves (presumably if there's
another antenna of the same frequency somewhere on the map. No idea how this makes physical sense, but there you are...) and
we need to continue our antinodes beyond order 1 until we're outside the map.

Since we already had a "outside the map"-filter in place from task 1, the lazy solution was to calculate the antinodes
up to an order, that is _definitely_ outside the map in order to capture all possible nodes (hence why calculating the the 50th
node, even if that could never be on the map). 

This also pairs antennas with themselves, so the inner loop in `get_antinodes` now starts at the index of the outer node.

```
[1,2,3] => [[1,1], [1,2], [1,3], [2,2], [2,3], [3,3]]
```