use std::ffi::OsStr;
use crate::utils:: { telegram , byte_to_readable_format };
use sysinfo::{Disk, DiskExt, System, SystemExt};

use byte_to_readable_format::convert_byte_to_readable;
use telegram::send_telegram_message;

const MEMORY_THRESHOLD: u64 = 1024 * 1024 * 1024;
pub async  fn monitor_disk_usage () {

  let mut  sys = System::new_all();
    sys.refresh_all();
    let mut report = String::new();
    let mut trigger_warning = false;

    for disk in sys.disks() {
        let available: u64 = disk.available_space();
        let total: u64 = disk.total_space();
        let disk_name = disk.name().to_str().unwrap();
        let disk_is_removable: bool = disk.is_removable();

            // if available > MEMORY_THRESHOLD && disk_is_removable {
            //    println!("{} is removable", disk.name().to_str().unwrap())
            // }

            if available < MEMORY_THRESHOLD {
                println!("{} is available", disk.name().to_str().unwrap())
            }


    }
}
