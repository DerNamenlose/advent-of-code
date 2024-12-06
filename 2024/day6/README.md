# Solution Day 6

This time 2D navigation. Love it...

Setting up the infrastructure first: loading the map, turning it into something that can easily be navigated in 2D (i.e. _NOT
`Vec<String>`, because it's basically impossible to access a single character in a string by index) and set up the start position.
What's missing then is a simulation function, that steps the guard by 1 field according to the rules of the game or returns
`None`, if the guard has left the field.

This infrastructure takes up a significant amount of space in the code, but makes the actual implementation easier.

## Task 1

I chose a rather straightforward solution: simulate the guard until it leaves the field and mark every step with an `X`. When
finished, count the number of X on the field and you're done. 

_Note:_ This does assume, that there are no loops for the guard in the input, which luckily is the case. Otherwise we would have
needed some kind of marking algorithm like in task 2 to prevent us from being stuck in an endless loop (and the task wouldn't make sense).

## Task 2

This one was more tricky. The outer loops just try every possible position for a new obstacle and then run the algorithm to check, whether
the guard enters a loop.

I initially thought about a simple marking algorithm for loop detection: mark every field where I take a turn and stop, when
I take a turn on that field again, assuming I was in a loop then. While this type of algorithm works reasonably well to check, whether
a graph is acyclic, it returns too many results for this particular problem (2194 to be precise in my input, instead of the correct 1602).
Reason being: I can _turn_ on a field where I took a turn before without entering a loop, _as long as it's not the exact same turn_. 
So, the following is possible and not a loop:

```
.....
..#..
..+#.
..|..
..^..
..|..
```

Explanation: The guard goes up, hits the obstacle, turns to the right, immediately hits another obstacle, turns down and goes back the
was he came. Since my algorithm would mark the `+` field on the first turn, the immediate second turn would be classed as entering a
loop, even though the guard would end up facing a different direction afterwards.

To fix this, I need to track, whether I've made the exact same turn before. So, instead of making my map structure more complex to
record all turns made somewhere, I just copy and save every turn step (position and new direction). If I make a turn and find, that
this step is already in the list of previous turns, I've actually entered a loop and can stop. This then leads to the correct number
of possible obstacle postions.

After my son challenged me to optimize the thing a bit, the program got it's pre-check to only ever simulate the guard walk for positions
that are actually reachable on the original map and I took the chance to play with [rayon](https://github.com/rayon-rs/rayon). Initially
the algorithm placing the potential obstacles was just two nested loops. Replacing the inner loop with a rayon parallel iterator and
thereby distributing it to multiple CPU cores