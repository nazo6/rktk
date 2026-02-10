use std::process::{Command, Stdio, exit};

use anyhow::Context as _;
use dialoguer::Select;
use serialport::SerialPortType;

fn main() -> anyhow::Result<()> {
    let serials = serialport::available_ports().context("Failed to find aviable serial ports")?;
    let usb_serials = serials
        .into_iter()
        .flat_map(|s| {
            if let SerialPortType::UsbPort(info) = s.port_type {
                Some((info, s.port_name))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut usb_serials_with_name = vec![];
    for device in rusb::devices()
        .context("Failed to find usb devices")?
        .iter()
    {
        let device_desc = device.device_descriptor().unwrap();
        let pid = device_desc.product_id();
        let vid = device_desc.vendor_id();

        let matched_serial_port = usb_serials
            .iter()
            .find(|d| d.0.pid == pid && d.0.vid == vid);

        if let Some(sp) = matched_serial_port {
            match device.open() {
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to open USB device PID: {:04x}, VID: {:04x}: {}",
                        pid, vid, e
                    );
                    usb_serials_with_name.push((
                        pid,
                        vid,
                        sp.1.clone(),
                        "<unknown product>".into(),
                    ));
                }
                Ok(handle) => {
                    if let Ok(product) = handle.read_product_string_ascii(&device_desc) {
                        usb_serials_with_name.push((pid, vid, sp.1.clone(), product))
                    }
                }
            }
        }
    }

    let port = match usb_serials_with_name.len() {
        0 => {
            anyhow::bail!("No serial port found");
        }
        1 => &usb_serials_with_name[0],
        _ => {
            let selection = Select::new()
                .with_prompt("Choose serial port")
                .items(
                    usb_serials_with_name
                        .iter()
                        .map(|d| format!("{} (PID: {:04x}, VID: {:04x})", d.3, d.0, d.1))
                        .collect::<Vec<_>>(),
                )
                .interact()
                .context("Failed to read user input")?;
            &usb_serials_with_name[selection]
        }
    };

    println!(
        "Selected serial port: {} (PID: {:04x}, VID: {:04x}, Port: {})",
        port.3, port.0, port.1, port.2
    );

    let elf_config_path =
        dirs::config_dir().map(|p| p.join("rktk-defmt-print").join("elf-path-preset.csv"));
    let elf_path = if let Some(path) = &elf_config_path
        && path.exists()
        && let Ok(content) = std::fs::read_to_string(path)
    {
        let mut elf_path = None;
        for line in content.lines() {
            if let Some((name, path)) = line.split_once(",")
                && name == port.3
            {
                elf_path = Some(path.to_string())
            }
        }
        elf_path
    } else {
        println!(
            "No ELF path preset found. Config file path: {:?}",
            elf_config_path
        );
        None
    };

    let elf_path = if let Some(elf_path) = elf_path {
        println!("Using elf path '{}' from preset", &elf_path);
        elf_path
    } else {
        dialoguer::Input::<String>::new()
            .with_prompt("Path to ELF file")
            .default("target/thumbv7em-none-eabihf/debug/your_project".into())
            .interact_text()
            .context("Failed to read user input")?
    };

    let args: Vec<String> = std::env::args().collect();
    let extra_args = if let Some(pos) = args.iter().position(|a| a == "--") {
        args[pos + 1..].to_vec()
    } else {
        vec![]
    };

    let mut args = vec![
        "-e".into(),
        elf_path,
        "serial".into(),
        "--path".into(),
        port.2.clone(),
        "--dtr".into(),
    ];
    args.extend(extra_args);

    println!("Running 'defmt-print {}'", args.join(" "));

    let status = Command::new("defmt-print")
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    match status.code() {
        Some(code) => exit(code),
        None => {
            exit(1);
        }
    }
}
