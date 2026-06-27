# Week 1 Exercises

There are three exercises to get first-hand experience with the concepts from the first lecture session.

## Technical Setup

The exercises are intended to be solved without needing knowledge about Rust project setup, toolchain installation, or IDEs.
For each exercise template in [templates](./templates), copy the content of the template and paste the lines into a [Rust Playground session](https://play.rust-lang.org/).
The `main` function takes over the job of unit tests and checks your implementation.
You can temporarily replace the contents of the `main` function with your own code for debugging.

## Exercises

The exercise are indepent of each other and you can solve in any order, but we recommend the order that the following task descriptions are in.

### `fibonacci` Exercise

Replace the `todo!()` with your implementation of the Fibonacci function, which maps:
- n -> f(n)
- 0 -> 0
- 1 -> 1
- 2 -> 1
- 3 -> 2
- 4 -> 3
- 5 -> 5
- ...

### `color_counting` Exercise

Replace the `todo!()` with an implementation that can walk through an array of `ColorEntry`s and sum up the counts of each `Color`.

### `cpu` Exercise

Your task is the implementation of a simple CPU, which is commonly referred to as "stack machine".
The machine should have a stack with space for five `u64` values.
The machine should support the four instructions `Push`, `Pop`, `Add`, `Multiply`.
The documentation of the `Instruction` type explains the expected CPU behavior for each instruction in detail.

Your tasks:
- Implement a type `Cpu` that stores all the requried CPU state and that derives at least `Debug`.
- Replace the first `todo!()` with code that creates a fresh CPU with empty stack.
- Replace the second `todo!()` with an implementation that emulates the instructions as the documentation specifies.

## Possible Solutions

Suggestions for solutions are available at [solutions](./solutions).
Note that this example implementation is not a showcase for idiomatic Rust code.
The code is restricted to use only Rust language features taught in the first four hours of the lecture.
