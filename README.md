[![Build Status](https://travis-ci.com/Gabie-of-the-Bo/Rust-Tin-Language.svg?branch=develop)](https://travis-ci.com/Gabie-of-the-Bo/Rust-Tin-Language) [![codecov](https://codecov.io/gh/Gabie-of-the-Bo/Rust-Tin-Language/branch/develop/graph/badge.svg?token=6D89914D05)](https://codecov.io/gh/Gabie-of-the-Bo/Rust-Tin-Language)

# Tin Language
Tin language reference implementation in Rust _(see previous [Python implementation](https://github.com/Gabie-of-the-Bo/Tin-Language))_.

**Tin** is a Turing complete, stack-based programming language whose main points are being concise, extensible and powerful. Since its code is tiny, it can be ideal for some code golf challenges and complex calculations.

# Main features

Tin features the following:
* Complex mathematical functions (_Python's NumPy_ style).
* Full unicode support for custom operations.
* Compact flow control operations.
* Regex-based function naming system to allow complex expressions.

This language aims not to be specialized, so you can expect some intrincate statistics module or things like that to be developed, but they **will not** be part of the official standard function list. Feel free to open an issue if you think an specific function should be added to the standard list and it will be reviewed as an option if it is generic enough.

# Tutorial
_Coming soon :)_

# Examples

**Naive primality test:**
```
→n(.nι``.n%∀1.n>)∀←n

→n           Define the variable 'n'
  (          Start a storer block
    .nι      Create a vector from 0 to 'n'
    ``       Drop the first two values
    .n%      Divide all the values by 'n' and take the remainder
    ∀        Check if they are all different to zero (first value)
    1.n>     Additionally, check if n is larger than 1 (second value)
  )          End the storer block. A vector of size 2 will be added to the stack
∀            Check if both values are different to zero
←n           Drop the variable 'n'
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
