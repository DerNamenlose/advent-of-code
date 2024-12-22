# Solution Day 22

One of the few later ones I actually did on the day. The pseudo-random number generator is easy enough to
implement. If you read correctly, that is… I read "round to the nearest integer" first and was slightly off 
with my numbers, until I saw that little "down" in there, which makes the whole thing _much_ easier. You don't
actually need to round, you can just do integer division, which will cut off the fractional part, leaving you
with the already rounded-down number. 

## Task 1

After implementing the pseudo-random generator, it's easy enough to run it 2000x for each start number and
sum up the resulting numbers. Nothing special to see here.

## Task 2

This one was more complex. My initial naïve approach was to calculate all 4-digit sub-sequences, search for them
in all sequences and sum up the assiciated price. While this would probably yield the result _at some point_, the
complexity is _O(n² * m²)_ (with _n_ being the number of starting numbers and _m_ being the sequence length for each
starting number, assuming I got my maths right). Let's just say: not fun, definitely too slow.

My less naïve approach involves pre-calculating the first price of each sub-sequence for each starting number and
then afterwards going through them, summing them up grouped by sub-sequence and finding the maximum number of those
sums. This involves the heavy use of `HashMap`s (to keep track of the found sub-sequences and their associated number),
but brings complexity down to _ O(n * m)_ (for all three steps, actually. Pre-calculating the sequences and prices is _O(n * m)_,
calculating the sums and then finding the maximum is as well). This is then actually fast enough to finish before I
get bored. 