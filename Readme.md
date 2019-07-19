# Apothema

Apothema is a Rubik's cube modeling framework, which provides multiple features
useful for writing Rubik's cube solvers.

Some of the features are:
- Coordinate systems:
    - Represent edge / corners orientations / permutations as an integer;
    - Represent the UD slice edges as a permutation, sorted or not;
    - Represent the full cube as a tuple of integers;

So far it is pretty much ready for use. There might be some bugs/errors out
there tought. Also, some more useful coordinate systems for use with Kociemba's
method is missing.

## Performance

Solving a scrambled cube which required 7 moves takes about 200 seconds on a
mid 2015 Macbook with an 2.2 GHz Intel Core i7 and it eats a lot of RAM. This,
considering that a raw BFS search is being used, it not too bad. So far there
are no prunning tables.
