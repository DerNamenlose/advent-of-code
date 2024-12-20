# Solution Day 19

The full solutions are getting rarer and rarer. Today's topic was building strings from a fixed set of substrings.

## Task 1

The initial solution to task 1 stopped after finding one solution (see the commented out code for `is_pattern_possible(...)`).
My main idea was checking, whether a substring (towel) started the pattern and then recursively check whether the remaining
pattern could also be formed. If so, the pattern would be valid.

Initially I ran into an endless loop with one of my inputs. Reason being: the recursive check lead to a situation, where the
implementation would check the same invalid suffix again and again, every time finding a way to start it, but not finish it.
I could solve this by tracking the known bad suffixes in a separate `HashMap` and short-circuiting before entering the
recursive loop again.

## Task 2

Task 2 looks like a slight extension of Task 1 (just count all combinations instead of stopping at the first), but it proved
to be much harder initially. With my approach of tracking the known bad suffixes, I simply could not prevent the endless
loop even at the first input. Even now I'm not 100% sure, why. I'm not even sure, whether this is really an endless loop or just
very, very inefficient checking the same suffix again and again.

After some more thought I decided to switch to a memoization technique: for each string store the number of known solutions and
short-circuit when you already know the solution. This did the trick and I got my solution. This way I'm even able to express
Task 1 in terms of the solver for Task 2: a valid pattern is any pattern, where the number of possible combinations is greater 0.