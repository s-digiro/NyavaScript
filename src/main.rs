use toylisp::*;
use std::env;

pub fn main() -> Result<(), String> {
    let text = env::args().skip(1).collect::<Vec<String>>().join(" ");
    println!("in: {}", text);
    println!();
    let res = run(&text);

    match res {
        Ok(res) => {
            let s = res.to_string();
            println!("{}", if res.to_string().starts_with("(") { trim(&s) } else { &s });
        },
        Err(e) => println!("{}", e),
    }

    Ok(())
}

fn trim(s: &str) -> &str {
    if s.len() > 1 {
        &s[1..s.len() - 2]
    } else {
        s
    }
}
