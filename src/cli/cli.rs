use std::str::FromStr;

use clap::{Parser, Subcommand, arg, command};

use crate::{
    api::{self, AdhanError, PrayerTimesPeriod},
    cli::ui,
};

#[derive(Parser)]
#[command(name = "Adhan Cli")]
#[command(version = "0.5")]
#[command(about = "Adhan Cli, Shows all prayers")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Show { city: String, period: String },
}

pub async fn init() -> Result<(), AdhanError> {
    let cli = Cli::parse();
    match cli.command {
        Command::Show { city, period } => {
            print!("Fetching");
            let period = PrayerTimesPeriod::from_str(&period)?;
            let data = api::get_prayer_data_by_city(&city, period).await?;
            let parsed = data.parse()?;
            print!("UI");
            let _ = ui::entry(parsed);
        }
    }

    Ok(())
}
