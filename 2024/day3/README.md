# Solution Day 3

This one felt much easier. Some "parsing" (well, not really, but anyway), instead of parsing and manipulating CSV.

## Task 1

The initial (seemingly) stupid idea proved to actually work: just use regex! Even got the regex right on the first
attempt. The stars must've aligned today. Just run through the matches, extract and parse the capture groups, multiply
and sum up.

## Task 2

Two complications: more complex regex with 3 different cases (still easy enough and correct in the first attempt) and
some tracking of state (whether the multiplication is currently enabled or not). Still way easier than day 2.