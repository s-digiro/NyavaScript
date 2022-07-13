# To Do
[ ] Strings
    [ ] Parse
        [*] Lexical
        [ ] Syntax
        [ ] Semantic
    [ ] Evaluate
[ ] Numbers
    [ ] Parse
        [*] Lexical
        [ ] Syntax
        [ ] Semantic
    [ ] Evaluate
[ ] Rust Contexts
    [ ] Move Special syntax parses to rust contexts if possible
[ ] Change how lists are evaluated
    1. Evaluate second through last
    2. If first is defined as a Lambda, execute it with second through last as args
[ ] Change how atoms are evaluated
    1. If they are not defined, evaluate them to NIL, not themselves
[ ] McCarthy things need to be implemented
    [ ] Car
    [ ] Cons
    [ ] Cdr
    [ ] Equal
    [ ] Maybe more
    [ ] McCarthy context??? as a type of Rust context?
[ ] Lib user defined Rust Contexts
[ ] Rust context of lib that I find useful
    [ ] Like print
    [ ] Add/Subtract
    [ ] Stuff that isn't pure McCarthy
