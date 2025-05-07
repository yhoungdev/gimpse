mod monitor;
mod utils;

use monitor::{battery_monitor, cpu, disk};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match battery_monitor::get_battery_info_pretty().await {
        Ok(battery_status) => println!("{}", battery_status),
        Err(e) => eprintln!("Battery monitor error: {}", e),
    }

    cpu::monitor_cpu().await;
    disk::monitor_disk_usage().await;

    Ok(())
}
