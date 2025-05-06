use crate::utils::telegram;
use battery::{units::thermodynamic_temperature::degree_celsius, Battery, Manager};

pub async  fn get_battery_info_pretty() -> Result<String, battery::Error> {
    let manager = Manager::new()?;
    let mut batteries = manager.batteries()?;

    let battery = match batteries.next() {
        Some(result) => result?,
        None => {
            return Err(battery::Error::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No battery found on this system",
            )))
        }
    };

    let percentage = battery
        .state_of_charge()
        .get::<battery::units::ratio::percent>();
    let temperature = battery
        .temperature()
        .map(|t| t.get::<degree_celsius>())
        .map(|t| format!("{:.1}°C", t))
        .unwrap_or("N/A".into());

    let voltage = battery.voltage().value;
    let energy = battery.energy().value;
    let energy_full = battery.energy_full().value;
    let energy_rate = battery.energy_rate().value;

    let time_to_empty = battery
        .time_to_empty()
        .map(|t| format!("{:.0} mins", t.value / 60.0))
        .unwrap_or("N/A".into());

    let mut report = format!(
        "🔋 Battery Info:
- State: {:?}
- Charge: {:.2}%
- Temp: {}
- Voltage: {:.2} V
- Energy: {:.2} Wh / {:.2} Wh
- Energy Rate: {:.2} W
- Time to Empty: {}",
        battery.state(),
        percentage,
        temperature,
        voltage,
        energy,
        energy_full,
        energy_rate,
        time_to_empty,
    );

    if percentage < 20.0 {
        telegram::send_telegram_message(" 🪫 Your Mac Battery is low! Please charge 🔌").await.expect("Failed to call telegram function on battery");
    }

    Ok(report)
}
