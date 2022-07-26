# To Do
[ ] Consider returning data type errors rather than evaluating errors to nil
[ ] Add tests to make sure McCarthy scope contains what we expect it to
[ ] Pass env to macro
[ ] Add line and column to errors where applicable
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
    [*] print cons cells with non-list cdr using dot infix
        [*] implement
        [*] Add tests
    [ ] Add . token
        [ ] implement
        [ ] test?
    [ ] Add . symbol to lexical parsing
        [ ] implement
        [ ] Add tests
            [ ] lone . parses into dot operator
    [ ] Add . Syntax
        [ ] implement
        [ ] Test?
    [ ] Add . symbol to syntax parsing
        [ ] implement
            [ ] Should parse into JUST Dot, not a tree
        [ ] Add test
             [ ] lone Dot Token parses into Dot Syntax
    [ ] Add . SExpr
        [ ] implement
        [ ] Test
            [ ] lots...
    [ ] Add . symbol to semantic parsing
        [ ] Parse it into new infixDot SExpr type
        [ ] Add test
            [ ] lone Dot Syntax parses into Dot Operator
    [ ] Add rules for how . SExpr is evaluated
        [ ] implement
        [ ] Test
            [ ] lots...
    [ ] Add some sort of e2e tests or lib unit tests
        [ ] Figure out the difference and standard practices while you are at it
[ ] Garbage collector
[ ] Evaluate List should be refactored into more functions so its easier to read
[ ] Allow multiple lists in root, without tacking on a root list as a new
    function. Maybe as a vector of refvals
[ ] Maybe strings should be lists of chars? Or maybe permanently quoted symbols?
    Pick one
[ ] List macro
[ ] Label macro
[ ] let macro
[ ] arbitrary cdaddr function
[ ] Maybe mccarthy functions should be "specials", separate from macros?
    decide.
[*] Invalid function calls should fail spectacularly rather than returning nil
[*] NIL keyword
[ ] From SX for SXRef
[ ] From inner for SXRef
[ ] From inner for SX
[ ] Better toString for Function
[ ] Better toString for Macro
[ ] Find a way to implement PartialEq on RustFunction
[ ] Find a way to implement PartialEq on RustMacro
[*] Move closures in McCarthyScope to their own functions?
