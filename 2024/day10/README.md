# Solution Day 10

Hillclimbing in software. Today is all about recursion. In order to find a way from a trailhead (a `0`) to
a peak (a `9`), we can just recursively look for the next step up until we either leave the map, cannot go
further (because the jump is too big or too small) or we've reached a peak.

If we cannot go further or leave the map, we've obviously not reached a peak, so the number of found peaks
is 0 in that step. If we found a peak, we can count it. The recursion then just checks in all 4 directions
from the current point and sums up the reachable peaks.

_Note:_ Technically checking in all 4 directions isn't necessary, since we just came from one of those directions.
However, in oder to simplify the algorithm, we can just check in all 4 and rely on the fact, that the recursion
will definitely stop in the direction we're coming from, because this will be downhill, which we don't allow.

## Task 1

Easy enough. Just execute the algorithm above and remove the peaks when you reach one (hence the copy of the 
heightmap). Removing the peak guarantees, that we don't count it more than once, if there are multiple
ways to reach it (guess what I did wrong at first). Disadvantage: we need a fresh copy of the map for each time
we start the wayfinding process.

Alternatively we could remember the positions of the peaks we already reached and check against that list when
we found them. Slightly more complex implementation (although with a `HashSet<(i32, i32)>` probably easy enough),
but more efficient because we won't need to clone the map again and again.

## Task 2

I feel like this is actually the simpler tasks of the two. The only change is, that we _don't_ mark the peaks
we've already visited, because this time we specifically _want_ to find all possible ways. Technically we could
technically get around the need to clone the height map, the signature of the `find_ways()` function requires
a mutable reference (even though we're not actually mutating the map in this use case), so the borrow checker
yells at us, if we borrow the map immutable (through the `.iter()` call) and mutable at the same time, so we just
copy it unnecessarily. The problem size does allow for that.