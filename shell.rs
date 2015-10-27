use console;

pub fn handle(cmd:&str) {
    let console = console::Console::init();
    if cmd.starts_with("ls") {
        console.print_string(".\n");
    } else {
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
