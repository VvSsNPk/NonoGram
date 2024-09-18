# Git Repository for NonoGram
In this assignment we need to solve a nonogram logic puzzle using a sat solver
the solver that i used is a rust based SAT solver called varisat which has a nice
API in rust natively to call the solver and solve any CNF formaula It provides very
nice helper functions to parse and dump the cnf formula which can be used to
analyze and debug your program later.

> Note: I did not bench mark the sat solver but it seems to be a CDCL sat solver
It ran on my machine nicely, but if you run the program and it is taking to long for it
to solve then you can try giving the raw dimacs file to another sat solver and verify my program

### How to run the Agent
To build the program you need run the following command
```bash
cargo build --release
```
to run the agent you have to give the following command
```bash
cargo run --release
```
when you run the program it will solve the nono gram and give you a solution file
as described in the systems project descripion you can the run the python script to check
and visualize the nonograms for correctness.

### The Main program is in the arch_linux branch of the Repo.

## Solution Summary
Here you can go to solution summary here [SolutionSummary](SolutionSummary.md)
