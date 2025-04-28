use anyhow::Result;

use crate::{cli::utils::device::is_keyboard, fs::device as fs};

pub fn list(keyboard: bool) -> Result<()> {
    let devices = fs::find_input_devices()?;

    for device in devices {
        if keyboard && is_keyboard(&device) {
            continue;
        }

        let name = device.name().unwrap_or("Unknown");
        let phys = device.physical_path().unwrap_or_default();
        let uniq = device.unique_name().unwrap_or_default();
        let input_id = device.input_id();

        println!("• {name}");
        println!("  ├─ Path      : {phys}");
        println!("  ├─ Unique ID : {uniq}");
        println!("  ├─ Vendor    : {:#06x}", input_id.vendor());
        println!("  ├─ Product   : {:#06x}", input_id.product());
        println!("  └─ Version   : {:#06x}", input_id.version());
        println!();
    }

    Ok(())
}
