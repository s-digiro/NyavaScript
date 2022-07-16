use toylisp::*;

pub fn main() -> Result<(), String> {
    let res = run("(\"hello\")");

    match res {
        Ok(res) => println!("{}", res),
        Err(e) => println!("{}", e),
    }

    Ok(())
}
