use std::io::Read;
pub fn command_help() {

    println!(r#"
"quit" - Exits the program.

"help" - Pulls up this screen.

"version" - Prints out the current version string.

"install" - Installs program to system.

Press enter to continue..."#);

    let buffer = &mut [0u8];
    std::io::stdin().read_exact(buffer).unwrap();
}