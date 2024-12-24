# Solution Day 24

Logic gate. What a treat. I couldn't fully solve it, though, as I got caught in thinking _way_ too complex. See below...

## Task 1

Rather straightforward. Each gate is represented by the `Gate` struct, which implements a `calculate` function, that either
gets the input values from the known inputs or recursivly calculates other gates to then calculate its own value. This
is slightly less efficient, than optimal, assuming we could cache intermediate values, but the calculation still is more
than fast enough.

## Task 2

Didn't solve this one because I thought way too complex. Looking at the input size, it was clear, that a brute-force search
for the correct swaps wouldn't be possible. I tried a few approaches to limit the search space. My main idea was, that no
swaps should be done for outputs, that are already correct (since we might change those) and that swaps should only be performed
between gates with different output values (otherwise we wouldn't change anything). After implementing the necessary preparations,
I discovered, that the search space is _still_ too large (I just let the commented out loop generating all the possible swaps
run without actually performing the swap, just to get a feel and was bored quite quickly of the runtime). After that I gave up
and searched for inspiration on reddit.

Turns out: my complex approach got me down the wrong path completely. If I had remembered my days as uni and the details
of what an adder looked like (I could still design one without looking it up. I just never thought of this circuit being
a standard parallel adder), I probably would've been able to find the swapped wires quite quickly. Well, you can't always
win, I guess…