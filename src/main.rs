use whoami;
use sysinfo::{System, SystemExt, DiskExt, NetworkExt, CpuExt};
use if_addrs::get_if_addrs;

fn print_whoami_info() {
    println!("User's Language: {:?}", whoami::lang_prefs().unwrap_or_default());
    println!(
        "User's Real Name: {}",
        whoami::realname().unwrap_or_else(|_| "<unknown>".to_string())
    );
    println!(
        "User's Username: {}",
        whoami::username().unwrap_or_else(|_| "<unknown>".to_string())
    );
    println!(
        "User's Account: {}",
        whoami::account().unwrap_or_else(|_| "<unknown>".to_string())
    );
    println!(
        "Device Pretty Name: {}",
        whoami::devicename().unwrap_or_else(|_| "<unknown>".to_string())
    );
    println!(
        "Device Hostname: {}",
        whoami::hostname().unwrap_or_else(|_| "<unknown>".to_string())
    );
    println!("Device Platform: {}", whoami::platform());
    println!(
        "Device Distro: {}",
        whoami::distro().unwrap_or_else(|_| "<unknown>".to_string())
    );
    println!(
        "Desktop Env: {}",
        whoami::desktop_env()
            .map(|e| e.to_string())
            .unwrap_or_else(|| "<unknown>".to_string())
    );
    println!("CPU Arch: {}", whoami::cpu_arch());
}

fn print_machine_info() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!(
        "OS: {}",
        sys.name().unwrap_or_else(|| "<unknown>".to_string())
    );
    println!(
        "Kernel version: {}",
        sys.kernel_version().unwrap_or_else(|| "<unknown>".to_string())
    );
    println!(
        "OS version: {}",
        sys.os_version().unwrap_or_else(|| "<unknown>".to_string())
    );
    println!(
        "Host name: {}",
        sys.host_name().unwrap_or_else(|| "<unknown>".to_string())
    );
    println!("Uptime (s): {}", sys.uptime());
    println!("Boot time (s since epoch): {}", sys.boot_time());

    println!("Total memory (KB): {}", sys.total_memory());
    println!("Used memory (KB): {}", sys.used_memory());
    println!("Total swap (KB): {}", sys.total_swap());
    println!("Used swap (KB): {}", sys.used_swap());

    println!("CPU count: {}", sys.cpus().len());
    for (i, cpu) in sys.cpus().iter().enumerate() {
        println!(
            " CPU {}: {}% (freq: {} MHz)",
            i,
            cpu.cpu_usage(),
            cpu.frequency()
        );
    }

    println!("Disks:");
    for disk in sys.disks() {
        println!(
            " - {}: {} available / {} total",
            disk.name().to_string_lossy(),
            disk.available_space(),
            disk.total_space()
        );
    }

    println!("Network interfaces (counters):");
    for (iface, data) in sys.networks() {
        println!(
            " - {}: received {} bytes, transmitted {} bytes",
            iface,
            data.received(),
            data.transmitted()
        );
    }

    // List interface addresses (IP) via if_addrs
    match get_if_addrs() {
        Ok(ifaces) => {
            println!("Interface addresses:");
            for iface in ifaces {
                println!(" - {}: {:?}", iface.name, iface.addr);
            }
        }
        Err(e) => println!("Failed to get interface addresses: {}", e),
    }

    println!("Process count: {}", sys.processes().len());
}

fn main() {
    let my_string = "bonjour";
    match my_string {
        "hello" => println!("world"),
        "Bonjour" => println!("tout le monde"),
        "hola" => println!("mundo"),
        _ => println!("unknown greeting"),
    }

    println!("--- whoami info ---");
    print_whoami_info();

    println!("\n--- system info (sysinfo) ---");
    print_machine_info();
}