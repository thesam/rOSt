use kernel::console::Console;
use sys;

pub fn run(console: &mut Console) {
    loop {
        console.print_string(sys::current_user());
        console.print_string("@");
        console.print_string(sys::hostname());
        console.print_string(":");
        console.print_string(sys::cwd());
        console.print_string("$ ");
        let foo = console.read_string();
        handle(console, foo.as_ref());
    }
}

fn handle(console: &mut Console, cmd:&str) {
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
