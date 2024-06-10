use std::path::Path;
use std::io::Read;
use std::process::{Command, exit};

pub fn version(var: &str) {

    println!("{}", format!("\"{var}\" is the current version string.").to_string());
    println!(
r"--Patch notes--
Added this section.
Added installer."
);
    println!("Press enter to continue...");
    
    let buffer = &mut [0u8];
    std::io::stdin().read_exact(buffer).unwrap();
}

pub fn install() {

    if Path::new("/usr/local/bin/DeeganTerm").exists() {
        
        println!("Program is already installed.");
    } else {

        //Runs the command "sudo cp DeeganTerm /usr/local/bin/DeeganTerm";
        let mut process = Command::new("sudo")
            .arg("cp")
            .arg("DeeganTerm")
            .arg("/usr/local/bin/DeeganTerm")
            .spawn()
            .expect("Command Failed To Run");

        let mut _result = process.wait().unwrap();

        process = Command::new("rm")
            .arg("DeeganTerm")
            .spawn()
            .expect("Command Failed To Run");

        _result = process.wait().unwrap();

        println!("Program will now exit.");
        exit(0);
    }
    println!("Press enter to continue...");
    
    let buffer = &mut [0u8];
    std::io::stdin().read_exact(buffer).unwrap();
}