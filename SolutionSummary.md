# Solution Summary for AI-1 systems project "Nono-Grams"
## Problem Definition
* In this assignment, we will use propositional logic to solve picture logical puzzles called nonograms.
* A SAT solver will handle propositional logic. We will be encoding the puzzle into a propositional formula and solve it using SAT solver.
* SAT solvers are powerful technology that takes a CNF formula and gives tells us whether the formula is satisfiable or not.
* CNF here means Conjunctive normal form i.e., Conjunction of literals
$$
(\neg A \lor B \lor C) \land (A \lor \neg B) \land (B \lor C)
$$
an example Cnf formula above
* So we will encode the whole logic puzzle as a CNF and we will then send it to a SAT solver

Now here I will go through different approached I have taken to solve the problem

## SAT solver and Various approaches used
### Varisat SAT interface in rust
* In this assignment, I used a CDCL based SAT solver called Varisat.
* The reason is that it has an interface with rust programming language which is my primary programming language to do my systems project
* I also want to write my own sat solver sometime later down the road in rust because unlike another languages rust ecosystem doesnot have a lot many sat solvers
### Various Approaches used to solve the problem
Before describing various approaches, I will explain the functionality of my API and how it handles different versions of the problem
In the pdf guide, several approaches we used to ease out the students and give them a starting point for the assignment
I followed the approaches 1 and approaches 2 to solve the assignment before going any further i will just discuss a little
bit about my code design for this assignment and how I designed a general nono gram to solve the problem which can be used for any approach.

The API is designed in such a way it can handle any kind of Nono-Grams. So in the API, essentially a NonoGram Struct is just nothing but
an array of Line structs this line describes different lines in different directions in NonoGram.
The line struct inturn has all the methods and functionality to add formulaes for the inbuilt Solver Interface.
There is functionality to add different information to SAT solvers as CNF formulaes. My program also dumps a dimacs file to test it with other sat solvers.
This could be used to benchmark other SAT solvers. I won't be going into another indepth functionality of my program.

The program does not convert anything from DNF to CNF explicitly, but it does it implicitly, i.e., without mentioning that its DNF and CNF. So if you look at the program
and are confused, that could be a reason behind it.

Now I will talk about different approach and how I solved them: -
#### Enumerating all possible variable assignments and trying to solve it that way. What does that mean exactly?
Consider a single cell like given below: -
> 2a 2a    | 1 | 2 | 3 | 4 | 5 |

so now we enumerate the possible assignments in propositional logic and get a DNF like below

$$
(1 \land 2 \land \neg 3 \land 4 \land 5)
$$

In the above case,
only one so assignment is possible, according to the rules, so the line is encoded in this way and sent to SAT solver
If I encode the whole Nono Gram with different lines, the sat solver will solve it and find a satisfying assignment.

Now comes the problem even though this approach is very straight forward. The problem was that it is not scalable.
The Maximum amount of size this approach could solve is a 20 x 20 size Nono Gram with a decent amount of time. Now then I
followed the Variant 2 of the suggested Approaches in the Guide.
#### Enclose Blocks Approach 
So after the first approach did not scale, I moved on to Variant 2 was this scalable?
* The problem with the first approach is the shear amount of DNF to CNF variables that gets added due to the tseitin transformations that we do.
* To eliminate this problem, we need to somehow reduce the number of variables. Which is what essentially this enclosed block variant does.
* In the first approach, we tell SAT solver about each and every single block separate as constraints to it to solve the SAT problem.
* But here instead of talking about single blocks, we enclose them i.e., pack them and say this pack of all the blocks is colored with respective clue colors.
* This will reduce the number of variables drastically. But there are some places where there is explosion of variables in this variant also i.e., in the XOR case.
* This approach solved all the problems in the given assignment.
* It scales well, but maybe the Last Variant State machines could work better? which unfortunately, I do not implement to test.
Now I will try to explain the encoding a little bit. This has already been done in the guide maybe I could add a little more explanation for multicolor variants. But I will include a detailed explanation in my Report.

## Summary
I solved all the NonoGRams and learned that how efficient encoding will change the problem in a different ways and how it could optimize the way sat solver solves the problem.
This could be improved further with further efficient encodings. In terms of scalability my programs scale well enough for it to solve all the Problems.
