use sysinfo::{Components, Disks, Networks, System};

use opencl3::device::{Device, CL_DEVICE_NAME, CL_DEVICE_VENDOR};
use opencl3::platform::{get_platforms, Platform};
use opencl3::types::{cl_device_id, cl_platform_id};

fn get_gpu_info() {
    // 获取平台列表
    match get_platforms() {
        Ok(platforms) => {
            for platform in platforms {
                // 打印平台名称
                println!("Platform: {}", platform.name().unwrap());

                // 获取平台上的设备（GPU/CPU）
                match platform.get_devices(opencl3::device::CL_DEVICE_TYPE_GPU) {
                    Ok(device_ids) => {
                        for device_id in device_ids {
                            // 获取设备名称
                            let device = Device::new(device_id); // Convert raw device to Device object

                            if let Ok(device_name) = device.name() {
                                println!("Device Name: {}", device_name);
                            }

                            if let Ok(device_vendor) = device.vendor() {
                                println!("Device Vendor: {}", device_vendor);
                            }

                            if let Ok(device_max_compute_units) = device.max_compute_units() {
                                println!("Device Max Compute Units: {}", device_max_compute_units);
                            }

                            if let Ok(device_global_mem_size) = device.global_mem_size() {
                                println!(
                                    "Device Global Memory Size: {} bytes",
                                    device_global_mem_size
                                );
                            }
                        }
                    }
                    Err(err) => eprintln!(
                        "Failed to get devices for platform {}: {}",
                        platform.name().unwrap(),
                        err
                    ),
                }
            }
        }
        Err(err) => eprintln!("Failed to get platforms: {}", err),
    }
}

fn get_system_info() {
    // Please note that we use "new_all" to ensure that all lists of
    // CPUs and processes are filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // Display system information:
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());

    // Display processes ID, name na disk usage:
    for (pid, process) in sys.processes() {
        println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
    }

    // We display all disks' information:
    println!("=> disks:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("{disk:?}");
    }

    // Network interfaces name, total data received and total data transmitted:
    let networks = Networks::new_with_refreshed_list();
    println!("=> networks:");
    for (interface_name, data) in &networks {
        println!(
            "{interface_name}: {} B (down) / {} B (up)",
            data.total_received(),
            data.total_transmitted(),
        );
        // If you want the amount of data received/transmitted since last call
        // to `Networks::refresh`, use `received`/`transmitted`.
    }

    // Components temperature:
    let components = Components::new_with_refreshed_list();
    println!("=> components:");
    for component in &components {
        println!("{component:?}");
    }
}

fn main() {
    get_system_info();

    get_gpu_info();
}
