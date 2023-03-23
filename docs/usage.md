# Usage

## Using The Interpreter
The interpreter is currently very simple.
There are no flags, just a filepath as input.

To run a file, invoke the interpreter with this command (linux):

    $ ./mspl [FILE_TO_RUN'

## The Stack
MSPL is a stack-based language, but what does that mean exacly?
A stack in a stack-based language is a data structure that holds values and operates on them using Last-In-First-Out (LIFO) semantics, where the most recently added item is the first to be removed.

### Pushing values to the stack
Pushing values to the stack is very easy, to push a number to the stack just enter a number, or for a character just enter a character in single quotes like:

    123 // Pushes 123 to the stack
    '0' // Pushes 48 (ascii value of '0') to the stack

### Stack manipulation
There are a total of 16 operators in MSPL.

- 7 boolean operators:  >, <, >=, <=, ==, &&, ||
- 5 bitwise operators:  >>, <<, &, |, ^
- 4 arithmetic operators: +, -, *, /

These operators perform on the top 2 values of the stack and then pushes the result. The operators can be used at any time by just typing an operator like:

    5 4 3 + * // Adds 4 plus 3 and pushes the result (7), multiplies 5 and 7 and pushes (35)

## Functions
Functions in MSPL are less like functions and more like macros, when calling a function expect your program to act you replaced where you called the function with the code in the function.

Functions can be defined by using the "def" keyword followed by the name of the function and then a code block. like:

    def myFunc {
        ...
    }

You can use a function simply just by saying the name of the function:

    myFunc  //runs the code in myfunc

### Predefined Functions
The interpreter already comes with a few functions defined.

1. 'not': Pops the top value off of the stack, if it's true (>0) pushes a 0, otherwise pushes a 1
2. 'dup': Duplicates the top value on the stack
3. 'swap': Swaps the top 2 values on the stack
4. 'rot': Swaps the top 3 values on the stack
5. 'print': Pops the stack and prints the integer form of that value
6. 'emit': Pops the stack and prints the ASCII form of that value
7. 'cr': Prints a newline character
8. 'in': Reads a number from stdin and pushes it to the stack

## Memory management
The MSPL interpreter currently has only 65536 words of memory, each word is a 32-bit signed value.

### Getting Memory
The '?' character takes the address at the top of the stack, and pushes the value at that address like:

    32 ? // Pushes the value at memory address 42 onto the stack

### Setting Memory
The '!' character uses the top value on the stack as the value, and the value below that as the address, and sets the value at that address to the value.

    32 123 ! // Sets memory address 32 to 123