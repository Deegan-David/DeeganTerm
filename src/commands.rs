use std::path::Path;
use std::io::Read;
use std::process::Command;

pub fn version(var: &str) {

    println!("{}", format!("\"{var}\" is the current version string.").to_string());
    println!(
r"--Patch notes--
Added this section.
Added install to system script.

"
);
    println!("Press enter to continue...");
    
    let buffer = &mut [0u8];
    std::io::stdin().read_exact(buffer).unwrap();
}

pub fn install() {

    if Path::new("/usr/local/bin/DeeganTerm").exists() {
        
        println!("File Exists");
    }

/*
    //Runs the command "sudo cp DeeganTerm /usr/local/bin/DeeganTerm";
    let mut process = Command::new("sudo")
        .arg("cp")
        .arg("DeeganTerm")
        .arg("/usr/local/bin/DeeganTerm")
        .spawn()
        .expect("Command Failed To Run");

    let _result = process.wait().unwrap();
*/
    println!("Press enter to continue...");
    
    let buffer = &mut [0u8];
    std::io::stdin().read_exact(buffer).unwrap();
}