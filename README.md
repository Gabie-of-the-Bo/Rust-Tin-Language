[![Build Status](https://travis-ci.com/Gabie-of-the-Bo/Rust-Tin-Language.svg?branch=develop)](https://travis-ci.com/Gabie-of-the-Bo/Rust-Tin-Language) [![codecov](https://codecov.io/gh/Gabie-of-the-Bo/Rust-Tin-Language/branch/develop/graph/badge.svg?token=6D89914D05)](https://codecov.io/gh/Gabie-of-the-Bo/Rust-Tin-Language)

# Tin Language
Tin language reference implementation in Rust _(see previous [Python implementation](https://github.com/Gabie-of-the-Bo/Tin-Language))_.

**Tin** is a Turing complete, stack-based programming language whose main points are being concise, extensible and powerful. Since its code is tiny, it can be ideal for some code golf challenges and complex calculations.

# Main features

Tin features the following:
* Complex mathematical functions (_Python's NumPy_ style).
* Full unicode support for custom operations.
* Compact flow control operations.
* Automatic functionwise parallelization.
* Regex-based function naming system to allow complex expressions.

This language aims not to be specialized, so you can expect some intrincate statistics module or things like that to be developed, but they **will not** be part of the official standard function list. Feel free to open an issue if you think an specific function should be added to the standard list and it will be reviewed as an option if it is generic enough.

# Tutorial
_Coming soon, just wait for the language to stabilize a bit :)_

# Examples

Simple factorial:
```
ι⊳∏

ι            Create a vector from 0 to the input 
⊳            Increment the numbers by one
∏            Multiply every number
```

Arithmetic mean:
```
!⍴↶∑/

!            Duplicate the last element of the stack
⍴            Get the length of the input vector
↶            Swap the last two element of the stack
∑            Sum the elements of the vector
/            Divide the sum by the length
```

Naive primality test:
```
→n.nι``.n%∀1.n>∧←n

→n           Define the variable 'n'
.nι          Create a vector from 0 to 'n'
``           Drop the first two values
.n%          Divide all the values by 'n' and take the remainder
∀            Check if they are all different to zero (first value)
1.n>         Additionally, check if n is larger than 1 (second value)
∧            Check if both values are different to zero
←n           Drop the variable 'n'
```

Recursive factorial:
```
◊⟨!!⊲∇·→n⟩:⟨1→n⟩.n←n

◊            Non-consuming skip. Skip next operation if the input is 0
⟨            Start code block. The full block is considered an operation
  !!         Duplicate the last element of the stack twice
  ⊲∇        Decrement the last element and call the program recursively
  ·          Multiply the last two numbers in the stack
  →n         Define the result as the variable 'n'
⟩            End code block
:            Else
⟨            Start code block
  1→n        Set variable 'n' to 1
⟩            End code block
.n←n         Return 'n' and clean up
```

Recursive fibonacci numbers:
```
!1<?⟨⊲!⊲∇↶∇+⟩

!            Duplicate the last element of the stack
1<?          Skip the next operation if the input is less than 1
⟨            Start code block
  ⊲!⊲       Decrement, duplicate and decrement again (generate input - 1 and input - 2)
  ∇         Call recursively for input - 1
  ↶∇        Swap the last two values in the stack and call recursively for input - 2 
  +          Sum fib(input - 1) and  fib(input - 2) 
⟩            End code block
```

# Project status
The current implementation is **usable**, but until coverage tests and some more extra functionalities are added this language might not be for you. Also, there is no binary release yet (but it is coming soon).

In order of relevance, these are the main tasks in progress:
* Standard library with sufficient mathematical functions.
* Extensive tests (coverage, function-wise unit testing...) to allow full confidence.
* Module system.
* Command-line execution.

In order of relevance, these are the main planned tasks for the future:
* Extended function naming capabilities.
* Custom **REPL** console
* IDE support, since it uses some unicode characters.
