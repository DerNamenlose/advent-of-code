# Solution Day 11

This one was a tricky one. I actually needed a bit of inspiration. The initial idea was rather straightforward: implement the simulation rules as a single step, simulate
them x times for all stones in the input and count the number of stones in the end. But...

## Task 1

This one is easy. Implement the simple approach outlined above, get the result. Nothing special, because it's "just" 25 rounds, so we're good. My original implementation
was actually done based on the string values, because two of the rules are easy enough this was: comparing with `"0"` is just as easy as using a numerical value,
splitting the number of even length is even easier. The only issue is correctly handling the leading `0`, so in the end I did just do everything with `u64`, only
converting to string in order to split (which could technically also have been done with clever use of `log10()`, `/` and `%`, but I'm lazy).

## Task 2

This is where it all came crashing down. I do have a _very_ beefy laptop with 32 GB of RAM and still my naive approach was killed by the OOM killer after significant
amounts of runtime. Since the stones can be simulated in isolation, we don't actually have to store all of the results, but rather can just count them, so I rewrote
the algorithm to work recursivly, calculating the count of a stone depth first. This means, that we will never have to store the actual stones (we'll just hand them
into the recursive call), which keeps the memory under control. It doesn't solve the runtime issue, though. In the iterative program I had printed the round index
at some point and that made it abundantly clear, that this program would probably run the whole night.

Here is where I needed a bit of inspiration. I read through a few of the posts under the hashtag #adventofcode on the Fediverse and someone mentioned memoisation issues
they were having. So, a cache it is then? I implemented a simple in-memory cache, that maps `(stone, run number)` => `count in the end`. In order to keep the cache
size under control, I decided to go with `stone`s < 1000 for a start (that would at most give us ~75,000 entries in the cache, which should be OK). Well, turns out that
this is plenty. The program finished so quickly, that I initially thought I had made a mistake with the cache and didn't actually want to try the result. Much to my
surprise (I honestly let out a little scream of surprise, much to my wife's surprise in turn), the result was correct. The small cache is so efficient, that the
calculation runs way, way below a second on my laptop. Someone smarter than me can probably calculate the cache hit rate and why this thing is so efficient.