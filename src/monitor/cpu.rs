use crate::utils::telegram;
use std::alloc::System;
use sysinfo::{Component, Cpu, Disk, Networks, System};

const TREASHOLD: f32 = 50.0;
async fn monitor_cpu() {
    let mut sys = System::new_all();
    let mut report = String::new();
    sys.refresh_all();
}
