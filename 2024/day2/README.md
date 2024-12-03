# Solution Day 2

Slightly upping the complexity, at least if you don't want to go the brute force route in task 2. 

Again reading a CSV, this time without a specified fixed amount of columns. The interesting bit is,
that we're not so much working with the numbers, but rather their differences, so we're basically in
for a instance of the [fencepost error](https://en.wikipedia.org/wiki/Off-by-one_error) if we get it
wrong. Hooray, I guess...

## Task 1

Task 1 is reasonably easy: check all the differences, see whether the series either always goes up or down
and whether it doesn't do so too quickly (steps between 1 and 3, inclusive). Just don't get the off-by-one 
wrong when calculating the differences and correct track, whether the series changes direction at any point.

## Task 2

This one felt reasonably tricky for the second day. Try removing one element from an invalid series from 
task 1 to make it valid again. In th end I tried 4 approaches:

1. Try skipping an element if it makes the series invalid (e.g. too big a step or going in the wrong direction).
   Misses things like `[3,2,5,7,9]`, where removing the `3` at the start would make the series valid. Since this
   approach would only ever skip the second element of the pair (i.e. the one the made the "wrong" step from the
   point of view of the rules), it would never ever remove the first element.
2. Check for each element, whether removing it makes a valid sequence. Basically the brute-force approach. Not
   very efficient, but get's the job done. Since the input is small enough, it's OK and was my solution until...
3. ...my wife proposed to just combine solutions 1. and 2. and only check the _two_ elements, that make the
   series invalid. Basically try removing either one and check whether it make a valid sequence. Works for the
   sequence in solution 1, but fails to correctly solve `[3,2,3,4,5]` (basically because it notices an issue
   at index `2` tries to remove it, which would make `[3,2,4,5]`, still invalid, or tries index `1`, making 
   `[3,3,4,5]`, still invalid).
4. A slight variation of solution 3: instead of checking the invalid index and _one_ back, also check _two_ back (if possible).
   This ensures, that we're catching this last corner-case. It catches all corner-cases in the given input (and I
   can't think of a case it would miss) and it is more efficient than solution 2. Solution is _O(n*m)_ for _n_ being the number
   of total series and _m_ being the average length of a series. This solution is _O(n)_, since it turns the element removal
   step into an _O(1)_ operation.