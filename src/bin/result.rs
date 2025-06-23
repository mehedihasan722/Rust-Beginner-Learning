enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err(format!("Invalid choice: {}", input)),
    }
}

fn main() {
    let choices: [&'static str; 3] = ["mainmenu", "start", "quit"];
    for choice in &choices {
        match get_choice(choice) {
            Ok(MenuChoice::MainMenu) => println!("You chose the main menu."),
            Ok(MenuChoice::Start) => println!("You chose to start."),
            Ok(MenuChoice::Quit) => println!("You chose to quit."),
            Err(e) => println!("{}", e),
        }
    }
}
