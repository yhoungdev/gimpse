mod monitor;
mod utils;

use monitor::{battery_monitor, cpu, disk, network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bat = battery_monitor::get_battery_info_pretty()?;
    println!("{}", bat);
    Ok(())
}
