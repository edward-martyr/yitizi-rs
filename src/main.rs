use std::{
    error::Error,
    io::{stdin, stdout, Write},
};
use yitizi::get;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn Error>> {
    println!("異體字 yitizi-rs v{}\n查詢異體字！輸入 q 退出。(Query for variant Chinese characters! Input q to quit.)", VERSION);
    stdout().flush()?;

    loop {
        print!(">>> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        // ctrl-d
        if input.is_empty() {
            println!();
            break;
        }

        let input = input.trim();

        let chrs = input.chars();
        let count = chrs.count();

        // empty input
        if count == 0 {
            continue;
        }
        // quit
        if count == 1 && input.chars().next().unwrap() == 'q' {
            break;
        }

        for c in input.chars() {
            print!("{:?}: {:?}, ", c, get(c));
            stdout().flush()?;
        }
        println!();
        stdout().flush()?;
    }

    Ok(())
}
