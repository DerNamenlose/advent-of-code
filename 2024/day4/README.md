# Solution Day 4

Hooray... 2D text analysis... My favorite thing! Not. I am just not good with those. Never was, probably never
really will be.

## Task 1

I might've gone of the wrong track, because this solution is just so complicated. I started out in 2D, i.e.
split the text into lines and tried from there, but got fed up with the complexity and reverted back to a
1D-approach with jump offsets to get from one line to the other. Basically the algorithm goes from the start
point and checks whether it is too close to one of the edges, I remove the jump offsets in this direction from
my jumps array. After that the jumps get executed 4 times, collecting the letters along the way to form the words.
The algorithm collects all words in all directions from each point in the text and then checks, which of those
fit the pattern. 

Sounds simply, hoo boy! how many times I overran the array bounds before I got all parts of that right!

## Task 2

Much, much simpler. Either the task was really simpler or my approach just fit way better. Basically I check every position,
whether it's an `A` and, if yes, I check the diagonals for `M.S` and `S.M `. Basically the whole implementation
worked on the first try. I nearly didn't want to continue after task 1, but this was a quick win after all.