/*
    ____                            ______                  
   / __ \___  ___  ____ _____ _____/_  __/__  _________ ___ 
  / / / / _ \/ _ \/ __ `/ __ `/ __ \/ / / _ \/ ___/ __ `__ \
 / /_/ /  __/  __/ /_/ / /_/ / / / / / /  __/ /  / / / / / /
/_____/\___/\___/\__, /\__,_/_/ /_/_/  \___/_/  /_/ /_/ /_/ 
                /____/
*/
use std::io::{self, Write};


fn init(flag: &String) {
    clearscreen::clear().expect("failed to clear screen");
    println!("##    ____                            ______                  ##");
    println!("##   / __ \\___  ___  ____ _____ _____/_  __/__  _________ ___ ##");
    println!("##  / / / / _ \\/ _ \\/ __ `/ __ `/ __ \\/ / / _ \\/ ___/ __ `__ \\##");
    println!("## / /_/ /  __/  __/ /_/ / /_/ / / / / / /  __/ /  / / / / / /##");
    println!("##/_____/\\___/\\___/\\__, /\\__,_/_/ /_/_/  \\___/_/  /_/ /_/ /_/ ##");
    println!("##                /____/                                      ##");
    println!("             Enter \"help\" for a list of commands.");
    print!("{}", flag);
    print!("> ");
    io::stdout().flush().unwrap();
}

fn main() {

    let mut flag = String::new();

    loop {
        let mut arg = String::new();

        init(&flag);
        flag = String::new();

        std::io::stdin().read_line(&mut arg).unwrap();
        arg.pop();

        match arg.trim() {
        
            "quit" => break,

            _ => flag = format!("\"{arg}\" is not a valid command.\n").to_string(),
        }

        //let _wait = std::io::stdin().read_line(&mut line).unwrap();
    }
}

/*
    cherry was here :3
*/