# To Do
[ ] Replace StringErrors with actual errors where applicable
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
    [ ] expression
        [ ] cons_cell
            [ ] iter
            [ ] mod
        [ ] meta
            [ ] lambda
            [ ] macro
            [ ] mod
            [ ] rust_lambda
            [ ] rust_macro
        [ ] atom
        [ ] list
        [ ] mod
    [*] parse
    [ ] lib
[*] Call of non procedure returns nil
[*] Eliminate automatic root list
[*] NoRootList Error when missing first char is no open parenthesis
[*] Analyze dead code
