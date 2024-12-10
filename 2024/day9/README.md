# Solution Day 9

Disk defragmentation... That's a thing I haven't seen in a long while. That brings back memories of sitting in front of a text
representation of my hard disk, seeing the little squares jump back and forth when the defragmentation program started to cluster related blocks together...
The fragmentation here is somewhat simpler, than the real ones back then and the stakes are also somewhat lower. No risk
of screwing up data, when you accidentally crash.

## Task 1
The approach is rather simple: run two pointers from the front and the back of the disk. Whenever the back pointer encounters an occupied block,
run the front pointer until you find an empty block, copy over the contents from the back end remove them there. Stop when the two
pointers meet in the middle. Afterwards calculate the checksum.

## Task 2
A bit more involved. My approach searches a file block from the back, counts its length, searches an empty block from the front
to find enough free space and moves the data over. Initially I managed to have an endless loop whenever a block could not be copied
at all, because the back pointer searches for the next non-free block. Since some files couldn't be moved out of the way, the back
pointer would immediately find the same file again and try to copy it, failing again and the whole cycle started over. This is 
easily solved by the 

```rust
else {
    movable_file_idx -= 1; // step before the file that we were unable to move
}
```

block in line 106ff, which steps just _before_ the file we looked at last. This time we stop when we reach file 0, because this
cannot ever be moved (on account of being at the start of the disk already).

This approach has complexity _O(n²)_, which can probably be massively improved (e.g. by finding all empty blocks in one pass,
recording their positions and sizes in a `HashMap` and then using the _O(1)_ lookups to find a suitable free block, when making the second
pass to find the actual file blocks. This would bring the complexity to _O(n)_, if I'm not mistaken.)