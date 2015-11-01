use kernel::console;

pub fn handle(cmd:&str) {
    let console = console::Console::init();
    console.print_string("You wrote:\n");
    console.print_string(cmd);
    if cmd.starts_with("ls") {
        console.print_string(".\n");
    } else if (cmd.starts_with("p")) {
        console.print_string(cmd);
    }
     else {
        console.print_string("404\n");
    }
}

pub fn current_user() -> &'static str {
    return "root";
}

pub fn hostname() -> &'static str {
    return "rOSt";
}

pub fn cwd() -> &'static str {
    return "/";
}