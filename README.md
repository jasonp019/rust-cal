rust-cal is a reimplementation of the standard Unix cal command in rust.

I also want to add additional features whilst retaining full backwards-
compatibility.  The main new feature is to support a range of months
for which you want to print the calendars.

For example:

```code
% rust-cal Feb-Mar 2023
   February 2023
Su Mo Tu We Th Fr Sa
          1  2  3  4
 5  6  7  8  9 10 11
12 13 14 15 16 17 18
19 20 21 22 23 24 25
26 27 28

     March 2023
Su Mo Tu We Th Fr Sa
          1  2  3  4
 5  6  7  8  9 10 11
12 13 14 15 16 17 18
19 20 21 22 23 24 25
26 27 28 29 30 31
```
