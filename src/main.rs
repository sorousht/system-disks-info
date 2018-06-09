extern crate sysinfo;

use sysinfo::SystemExt;
use sysinfo::DiskExt;

const KILOBYTE: u64 = 1024;
const MEGABYTE: u64 = 1024 * 1024;
const GIGABYTE: u64 = 1024 * 1024 * 1024;
const TERABYTE: u64 = 1024 * 1024 * 1024 * 1024;


fn main() {

    let mut system = sysinfo::System::new();
    system.refresh_all();

    for disk in system.get_disks() {
        let total: u64 = disk.get_total_space();
        let available: u64 = disk.get_available_space();
        let available_percentage: u64 = (available * 100) / total;
        println!("{}({}) - {}% ({})",
            disk.get_name(),
            get_in_readable_unit(total),
            available_percentage,
            get_in_readable_unit(available));
    }

}

fn get_in_readable_unit(value: u64) -> String {
    if value < KILOBYTE {
        return format!("{} Bytes", value);    
    } else if value < MEGABYTE {
        return format!("{} KB", value / KILOBYTE);
    } else if value < GIGABYTE {
        return format!("{} MB", value / MEGABYTE);
    } else if value < TERABYTE {
        return format!("{} GB", value / GIGABYTE);
    } else {
        return format!("{} TB", value / TERABYTE);
    }
}
