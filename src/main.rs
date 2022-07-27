use nyavascript::*;
use std::env;

pub fn main() -> Result<(), String> {
    let text = env::args().skip(1).collect::<Vec<String>>().join(" ");
    println!("in: {}", text);
    println!();
    let res = run(&text);

    match res {
        Ok(res) => {
            let s = res.to_string();
            println!("{}", s);
        },
        Err(e) => println!("{}", e),
    }

    Ok(())
}
