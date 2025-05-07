pub fn convert_byte_to_readable(param: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;
    const TB: u64 = 1024 * GB;
    const PB: u64 = 1024 * TB;

    let size = param as f64;

    if param >= TB {
        format!("{:.2} TB", size / TB as f64)
    } else if param >= GB {
        format!("{:.2} GB", size / GB as f64)
    } else if param >= MB {
        format!("{:.2} MB", size / MB as f64)
    } else if param >= KB {
        format!("{:.2} KB", size / KB as f64)
    } else {
        format!("{} B", param)
    }
}
