use std::env;

mod weather;
use weather::get_weather;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to SkimBot, a Command Line Tool");
    println!("Type list to see all the Available Commands");

    loop {
        let arg: Vec<String> = env::args().collect();
        match arg.as_str() {
            "list" => println!("W"),
            "weather" => get_weather().await?,
            "end" => break,
            _ => println!("Invalid Command"),
        }
    }

    Ok(())
}