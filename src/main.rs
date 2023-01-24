use nyavascript::*;
use std::env;

pub fn main() -> Result<(), String> {
    let text = env::args().skip(1).collect::<Vec<String>>().join(" ");
    println!("in: {}", text);
    let res = run(&text);

    match res {
        Ok(res) => {
            let s = res.to_string();
            println!("out: {}", s);
        },
        Err(e) => println!("error: {}", e),
    }

    Ok(())
}
