# Solution Day 5

Slightly more complex file format this time. It consists of two parts: "rules" or the form `x|y` and "updates", that look like `x,y,z,...`.
Luckily for parsing we can just go through the `.lines()` iterator one by one, check whether the line contains a `|` and, if so, parse
it into a rule struct. If not, we just continue splitting it into `Vec<u32>`. We can do this the simple way consuming each line to look
for the `|`, because there's an empty line between the blocks, that we can just throw away. 

After everything is parsed, the "real" work starts: writing a rule-interpreter, that checks, whether the updates are correct by the rules.
The solution is rather simple: for each element we check, whether any element, that should come _after_ it, actually comes before and the
other way around. If this isn't the case, the update is correct.

While the rule-checker is reasonably simple, it _does_ need to go through all rules twice for each element of the update. This could be
improved by indexing the rules in a `HashMap`based on their constituents and do an `O(1)` lookup. The runtime is so fast, however, that
this is not worth the effort.

## Task 1

This task is easy: find all correct updates (just a filter with the rule interpreter) and sum up the middle elements. Since all updates are
of odd length, the middle element always exists, so `.reduce()` can do quick work.

## Task 2

Slightly more complex: the *in*correct rules should be retrieved and fixed, before the middle element is supposed to be summed up. Took
me a while, but then I had a bright idea: the rules all together are nothing more than an ordering. If you interpret all rules against a
pair of update entries, you can tell which way around they are supposed to go, i.e. which one is less than the other. So we can use
the `.sort_by()` function from the standard library with a custom comparison function, that interprets all relevant rules and returns the
ordering. After sorting the elements, summing up the middle is the same as in task 1.

_Note:_ This assumes, that the full set of rules represents a total order. If it were to be a partial order, the resulting order would be
unspecified. Luckily for us, the designers made the ruleset a total order. 

This could again be more efficient, by indexing the rules once first and then do an `O(1)` lookup for all relevant rules instead of checking
_all_ rules for each call to the comparator. Again, the runtime is so quick, that this seems unnecessary optimization.

Also, there's a `.clone()` for the update vector in there, because I couldn't figure out how to tell the borrow checker, that I would be 
happy to fully consume the input while checking it. There's no need for me to touch any update twice, so borrowing isn't actually necessary.
I just didn't find the right combination of mutable vectors and `IntoIterator`s to make this clear to the borrow checker.