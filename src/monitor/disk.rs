use crate::utils::{byte_to_readable_format, telegram};
use sysinfo::{DiskExt, System, SystemExt};

use byte_to_readable_format::convert_byte_to_readable;
use telegram::send_telegram_message;

const MEMORY_THRESHOLD: u64 = 1024 * 1024 * 1024;

pub async fn monitor_disk_usage() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut report = String::new();
    let mut trigger_warning = false;

    for disk in sys.disks() {
        let mount_point = disk.mount_point().to_string_lossy();
        let file_system = std::str::from_utf8(disk.file_system()).unwrap_or_default();
        let disk_name = disk.name().to_string_lossy();

        if file_system.contains("tmpfs")
            || file_system.contains("devtmpfs")
            || mount_point.starts_with("/media")
            || mount_point.starts_with("/run/media")
            || disk.is_removable()
        {
            continue;
        }

        let available = disk.available_space();
        let total = disk.total_space();

        let available_readable = convert_byte_to_readable(available);
        let total_readable = convert_byte_to_readable(total);

        report.push_str(&format!(
            "Disk: {}\n  Mount: {}\n  FS: {}\n  Available: {} of {}\n\n",
            disk_name, mount_point, file_system, available_readable, total_readable
        ));

        if available < MEMORY_THRESHOLD {
            trigger_warning = true;

            let message = format!(
                "⚠️ Warning: {} (mounted on {}) is low on space. Available: {} out of {}",
                disk_name, mount_point, available_readable, total_readable
            );
            send_telegram_message(&message)
                .await
                .expect("Failed to send disk message");
        }
    }

    if trigger_warning {
        println!("Disk Usage Warning:\n{}", report);
    } else {
        println!("Disk usage is within acceptable limits.\n{}", report);
    }
}
