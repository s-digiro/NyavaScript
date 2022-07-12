use toylisp::*;

pub fn main() -> Result<(), String> {
    let res = run("
(defun xg (x) (x g))
(defun callOnY (x y) (x y))
(xg b)
(xg c)
(callOnY xg b)
    ");

    match res {
        Ok(res) => println!("{}", res),
        Err(e) => println!("{}", e),
    }

    Ok(())
}
