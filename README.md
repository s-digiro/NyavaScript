# To Do
- Features
    - [ ] A compiler (lol)
    - [ ] Pass env to macro
    - [ ] Add line and column to errors where applicable
    - [ ] Figure out the difference and standard practices between e2e tests and lib unit tests
    - [ ] From SX for SXRef
    - [ ] From inner for SXRef
    - [ ] From inner for SX
    - [ ] Better toString for Function
    - [ ] Better toString for Macro
    - [ ] PartialEq on functions
        - [ ] Rust: When function is created, create it as a singleton with a timestamp hash
            - [ ] Compare Hashes
        - [ ] Lisp: Same as with Rust
        - [ ] Make sure that's all of them
    - [ ] PartialEq on Macros
- DataTypes
    - [ ] Hashmaps
    - [ ] Vectors
    - [ ] Set
    - [ ] Float
- Functions
    - [ ] Sys calls somehow
    - [ ] load for loading stuff from other files
    - [ ] While
    - [ ] let
    - [ ] define
    - [ ] String util
        - [ ] print
        - [ ] println
        - [ ] string concatenation
        - [ ] string indexing
        - [ ] string length
        - [ ] string to list of chars
        - [ ] Check what's in libc and duplicate it
    - [ ] Math stuff
        - [ ] Math operations
            - [ ] Addition
            - [ ] Subtraction
            - [ ] Multiplication
            - [ ] Division
            - [ ] Modulo
            - [ ] Check what's in libc and duplicate it
    - [ ] bitwise operations
        - [ ] and
        - [ ] or
        - [ ] left shift
        - [ ] right shift
        - [ ] not
    - [ ] list
- Bugs
    - [ ] Passing lambdas to other lambdas is a lil weird... figure it out
- Refactor
    - [ ] Evaluate List should be refactored into more functions so its easier to read

# Potential features
- [ ] inline assembly????
- [ ] Fraction type?
- [ ] Array literal?
- [ ] Maybe root lists should execute concurrently?
- [ ] Concurrent macro?
- [ ] set? Maybe?
- [ ] Maybe strings should be lists of chars? Or maybe permanently quoted symbols? Pick one
- [ ] Maybe mccarthy functions should be "specials", separate from macros? decide.
- [ ] Byte data type
- [ ] Blob data type
- [ ] Fixed array data type
- [ ] Consider returning data type errors rather than evaluating errors to nil
- [ ] Garbage collector
