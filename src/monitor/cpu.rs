use crate::utils::telegram;
use sysinfo::{CpuExt, System, SystemExt};
const THRESHOLD: f32 = 50.0;

pub async fn monitor_cpu() {
    let mut sys = System::new_all();
    sys.refresh_cpu();

    let mut report = String::new();

    for (i, cpu) in sys.cpus().iter().enumerate() {
        let usage = cpu.cpu_usage();
        if usage > THRESHOLD {
            report.push_str(&format!(
                "⚠️ CPU Core {} is spiking: {:.2}% usage\n",
                i, usage
            ));
        }
    }

    if !report.is_empty() {
        telegram::send_telegram_message(&report)
            .await
            .expect("Failed to send message");
    }
}
