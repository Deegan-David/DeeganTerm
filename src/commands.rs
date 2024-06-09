use std::io::Read;

pub fn version(var: &str) {

    println!("{}", format!("\"{var}\" is the current version string.").to_string());
    println!("Press enter to continue...");
    
    let buffer = &mut [0u8];
    std::io::stdin().read_exact(buffer).unwrap();
}