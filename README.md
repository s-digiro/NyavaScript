# To Do
[ ] pass all args to macro
[ ] Pass env to macro
[ ] Add line and column to errors where applicable
[ ] (quote (1 2)) behaves differently from ('(1 2)) and it shouldn't
[ ] Passing lambdas to other lambdas is a lil weird... figure it out
[ ] McCarthy things need to be implemented
    [*] Car
    [*] Cons
    [*] Cdr
    [*] Equal
    [*] quote
    [ ] Maybe more
    [*] McCarthy context??? as a type of Rust context?
    [ ] Make sure all McCarthy things are added
[ ] Lib user defined Rust Contexts
[ ] Rust context of lib that I find useful
    [ ] Like print
    [ ] Add/Subtract
    [ ] Stuff that isn't pure McCarthy
[ ] . infix operator
[ ] Garbage collector
[ ] Add tests
    [ ] evaluate
        [ ] environment
            [ ] mccarthy_scope
            [ ] mod
        [ ] mod
    [*] s_expression
        [*] cons_cell
            [*] iter
            [*] mod
        [*] function
        [*] macro
        [*] rust_function
        [*] rust_macro
        [*] list
        [*] mod
    [*] parse
    [ ] lib
[ ] Lisp macro should only recieve one arg, which is arg list
[ ] Lisp lambda should get args like normal
[ ] Evaluate List should be refactored into more functions so its easier to read
[ ] Allow multiple lists in root, without tacking on a root list as a new
    function. Maybe as a vector of refvals
[ ] Everything should be implementable in lisp, other than Mccarthy basics
[ ] Maybe strings should be lists of chars? Or maybe permanently quoted symbols?
    Pick one
[ ] List macro
[ ] Label macro
[ ] let macro
[ ] arbitrary cdaddr function
[ ] Maybe mccarthy functions should be "specials", separate from macros?
    decide.
[ ] Invalid function calls should fail spectacularly rather than returning nil
[ ] NIL keyword
[ ] From SX for SXRef
[ ] From inner for SXRef
[ ] From inner for SX
[ ] Better toString for Function
[ ] Better toString for Macro
[ ] Find a way to implement PartialEq on RustFunction
[ ] Find a way to implement PartialEq on RustMacro
