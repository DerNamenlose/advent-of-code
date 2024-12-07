# Solution Day 7

Calculating expression and testing whether we can make them equal on both sides. Not too hard.
Parsing the input is rather easy with a bit of `.split()` and `.parse()`. Algorithm for testing the
expressions is of the brute-force variety: generate all possible combinations of operators, calculate
the expression for each combination and see if it matches the result. 

## Task 1

This one was easy at first: all possible combinations of operators (since there are only 2) can be generated
by counting between `0` and `pow(2, size)` (where `size` is the number of operators we need to fill the equation)
and treating each number as a bit mask, where the bit in position `n` represents an operator (`Add` if the bit
is `0`, `Multiply` if it is `1`). This was easy enough to implement and I even got it right on the first try.

## Task 2

I kind of had the inkling, that my simple "bitmask" solution for task 1 would get me into trouble with task 2. And
wouldn't you know? It did. Three operators cannot be easily expressed with a binary number (more on that in a bit).
I thought about implementing symbolic calculations (i.e. representing the number as `Vec<usize>` and doing the
counting by hand, including overflow and everything), but that seemed too much effort. The crate `itertools` contains
a `.combinations()` method, which can be made to work like this: `[Add, Add, Multiply, Multiply, Concat, Concat].combinations(2)`
(not the actual syntax, but you get the idea). This will generate all possible length 2 combination of the operators.
Works, but is _exceedingly_ slow, so I abandoned that approach. 

My actual solution goes back to my teaching days at uni: calculating the digits of a number in arbitrary base (in
this case: base 3). To get the next digit at the back, calculate `number % base`. Then continue with `number /= base` to
get the number without the last digit and continue until you run out of digits (or, in my case: until you've generate enough
digits corresponding to the operators needed).

Another tricky bit was the implementation of the `||` operator. Since the whole implementation is done numerically, it
_should_ be implemented as `left * pow(10, round(log10(right))) + right` (basically: multiply with 10 ^ number of right digits to 
get enough 0s at the end and add the number on top). While this works in theory, there is probably a precision or something, 
since `ceil` and `log10` only work in `f64` instead of `u64`. I consistently got the wrong results. In the end I gave up and
used a combination of `format!()` and `.parse()` to concatenate in string-space. 