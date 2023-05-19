use std::io::{self, BufRead};

mod weather;
use weather::get_weather;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let welcome_message = r#"
        Welcome to ElementaCLI, a Command Line Tool
        
        Available Commands:
        - list: Displays the available commands
        - weather [city]: Retrieves weather information for the specified city
        - end: Exits the program
        
        Enter a command:
    "#;

    println!("{}", welcome_message);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(input) = line {
            let mut args = input.trim().split_whitespace();
            match args.next() {
                Some("list") => println!("Available Commands:\n- list\n- weather [city]\n- end"),
                Some("weather") => {
                    if let Some(city) = args.next() {
                        get_weather(&city.to_string()).await?; // Await the future returned by get_weather
                    } else {
                        println!("Please provide a city argument for the 'weather' command.");
                    }
                }
                Some("end") => break,
                _ => println!("Invalid Command. Please enter a valid command."),
            }
        }
    }

    Ok(())
}