# To Do
- [ ] Sys calls somehow
- [ ] inline assembly????
- [ ] Rename s\_expression to form
- [ ] Rename function to procedure
    - [ ] Make sure this reflects in toString as well
- [ ] Control flow scope
    - [ ] while loop
    - [ ] let macro
    - [ ] define global var
- [ ] Basic scope
    - [ ] print
    - [ ] println
    - [ ] string concatenation
    - [ ] string indexing
    - [ ] string length
    - [ ] string to list of chars
    - [ ] Math operations
        - [ ] Addition
        - [ ] Subtraction
        - [ ] Multiplication
        - [ ] Division
        - [ ] Modulo
    - [ ] bitwise operations
        - [ ] and
        - [ ] or
        - [ ] left shirt
        - [ ] right shirt
        - [ ] not
- [ ] Consider returning data type errors rather than evaluating errors to nil
- [ ] Add tests to make sure McCarthy scope contains what we expect it to
- [ ] Pass env to macro
- [ ] Add line and column to errors where applicable
- [ ] Passing lambdas to other lambdas is a lil weird... figure it out
- [ ] McCarthy things need to be implemented
    - [x] Car
    - [x] Cons
    - [x] Cdr
    - [x] Equal
    - [x] quote
    - [ ] Maybe more
    - [x] McCarthy context??? as a type of Rust context?
    - [ ] Make sure all McCarthy things are added
- [ ] Lib user defined Rust Contexts
- [ ] Figure out the difference and standard practices between e2e tests and lib unit tests
- [ ] Garbage collector
- [ ] Evaluate List should be refactored into more functions so its easier to read
- [ ] Maybe strings should be lists of chars? Or maybe permanently quoted symbols? Pick one
- [ ] List macro
- [ ] Label macro
- [ ] let macro
- [ ] arbitrary cdaddr function
- [ ] Maybe mccarthy functions should be "specials", separate from macros? decide.
- [ ] From SX for SXRef
- [ ] From inner for SXRef
- [ ] From inner for SX
- [ ] Better toString for Function
- [ ] Better toString for Macro
- [ ] Find a way to implement PartialEq on RustFunction
- [ ] Find a way to implement PartialEq on RustMacro
