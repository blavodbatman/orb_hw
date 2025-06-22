mod commands;

fn main() {
    let mpass = commands::type_password("Enter master password:");

    if !std::fs::exists(commands::FILE_NAME).unwrap() {
        println!("Confirm master password:");

        let mpass_conf = rpassword::read_password().unwrap();
        if mpass != mpass_conf {
            println!("Passwords do not match. Goodbye!");
            return;
        }

        match std::fs::File::create(commands::FILE_NAME) {
            Err(err) => panic!("Couldn't create file {}", err),
            Ok(file) => file,
        };

        let Ok(_) = commands::save_mpassword(&mpass) else {
            println!("Oops! Something went wrong. Goodbye!");
            return;
        };
    } else {
        let check = commands::check_mpassword(&mpass);
        if check.is_err() || !check.unwrap() {
            println!("Access denied. Goodbye!");
            return;
        }
    }

    let mut command = Default::default();
    loop {
        println!("Enter command (type help to show available commands or type exit):");
        std::io::stdin().read_line(&mut command).unwrap();
        if let Err(e) = execute_command(&command, &mpass) {
            println!("{e}")
        };
        if command.trim() == "exit" || command.trim() == "delete file" {
            break;
        }
        command.clear();
    }
}

fn execute_command(command: &str, mpass: &str) -> Result<(), commands::Errors> {
    match command.trim() {
        "exit" => commands::exit(),
        "help" => commands::help(),
        "add" => commands::add(mpass)?,
        "show" => commands::show(mpass)?,
        "show all" => commands::show_all(mpass)?,
        "change" => commands::change(mpass)?,
        "delete" => commands::delete(mpass)?,
        "delete all" => commands::delete_all()?,
        "delete file" => commands::delete_file()?,
        _ => println!("Undefined command, try to type help."),
    };
    Ok(())
}
