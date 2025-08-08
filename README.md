# Adhan CLI 🕌

A beautiful terminal-based Islamic prayer times application built with Rust.

## Features

- 📅 Display prayer times for any city
- 🧭 Qibla direction compass visualization
- ⏰ Live countdown to the next prayer
- 🎨 Beautiful terminal user interface with color highlighting
- 📊 Support for daily, weekly, monthly, and yearly prayer schedules

## Installation

Make sure you have Rust installed on your system. Then clone and build:

```bash
git clone https://github.com/Abdo30004/adhan-cli.git
cd adhan-cli
cargo build --release
```

## Usage

Run the application with the `show` command followed by a city name and time period:

```bash
cargo run -- show <CITY> <PERIOD>
```

### Parameters

- `<CITY>`: Name of your city (e.g., `algiers`, `london`, `newyork`)
- `<PERIOD>`: Time period for prayer times
  - `daily` - Today's prayer times
  - `weekly` - This week's prayer times
  - `monthly` - This month's prayer times
  - `yearly` - This year's prayer times

### Examples

```bash
# Show today's prayer times for Algiers
cargo run -- show algiers daily

# Show weekly prayer times for London
cargo run -- show london weekly

# Show monthly prayer times for New York
cargo run -- show newyork monthly
```

## Interface

The application displays:

- **Header**: Current date, time, and location
- **Qibla Compass**: Visual representation of the Qibla direction
- **Prayer Times List**: All five daily prayers with times
  - Highlighted prayer indicates the next upcoming prayer
- **Next Prayer**: Countdown timer to the next prayer

### Controls

- Press `q` to quit the application

## Technologies Used

- **Rust** - Programming language
- **Ratatui** - Terminal UI framework
- **Reqwest** - HTTP client for fetching prayer times
- **Chrono** - Date and time handling
- **Clap** - Command-line argument parsing

## Data Source

Prayer times are fetched from the MuslimSalat API.

## License

This project is open source and available under the MIT License.
