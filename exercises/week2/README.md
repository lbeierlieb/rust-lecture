# Week 1 Exercise: Zombie Infection Simulation

Watch this [YouTube video](https://www.youtube.com/watch?v=g-g-NdyYwsY) for nice visualizations and explanations about the simulation this exercise aims to implement.
Short summary: The world is a grid of squares (in this exercise we use a fix world size of 100x100).
Initially, a grid cell is either uninhabited or a human lives on this cell.
In a world where 3000 of the 10000 cells are inhabited, the population density is 30%.

When we now infect one of the humans, and this zombie infects his direct neighbors, which again infect their direct neighbors.
The infection chain continues until either all humans in the world are zombies or until uninhabited cells separate the zombie crowd from the remaining humans.
When the world is densily populated, the former case is more likey, whereas in a sparsely populated world the latter case dominates.

We implement this simulation in Rust with the goal of determining how the population density affects the percentage of infections at the end of a simulation run.

## Technical setup

The exercise is intended to be solved without needing knowledge about Rust project setup, toolchain installation, or IDEs.
Copy the contents of [template.rs](./template.rs) and paste the lines into a [Rust Playground session](https://play.rust-lang.org/).

## Code from the Template

The template already specifies 


## Possible Solution

A suggestion for a solution is available at [zombiesim/src/main.rs](./zombiesim/src/main.rs).
Note that this example implementation is not a showcase for idiomatic Rust code.
The code is restricted to use only Rust language features taught in the first four hours of the lecture.
