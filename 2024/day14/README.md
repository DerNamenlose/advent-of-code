# Solution Day 14

Again one of those I couldn't do without inspiration. Reading the input is easily enough with the regex crate.
Simulating the robots is also very easy, although I got it wrong in the first try. I initially thought `x %= WIDTH`
is enough to bring `x` back into the allowed range, completely forgetting about negative numbers. After fixing 
this issue, the task is actually easy enough.

Task 2, however... "form the shape of a Christmas tree"... What? I didn't even have an idea what this would look like.
Just the outer edge? Filled? Centered? What means "most" line up to form the Christmas tree? I was at a complete
loss. Some inspiration from Reddit later, I assumed, that the Christmas tree might form an outline, centered around
`x = 50`, so I calculated the number of robots, where the distance from the center column was roughly half the
distance from the top of the map (basically any robot, that roughly lies on one of two lines falling
away from the center column). Didn't work at all. Not within the first 100,000 seconds.

After some more digging someone suggested, that the safety factor from task 1 might actually be the key. They
proposed to calculate the safety factor for each second and render the map every time a new minimum occured. This
assumed, that the robots would cluster together, leaving large parts of the map empty and therefore have a very
low safety factor. This does actually work. After a few false positives, the tree emerges in a rendered map.