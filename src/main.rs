use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use device_query::{DeviceQuery, DeviceState, Keycode};
use serde::{Deserialize, Serialize};
mod config;
mod service;

#[derive(Debug, Serialize, Deserialize)]
struct KeyPress {
    key: String,
    count: usize,
}

pub fn save_keypress(key_count: &HashMap<Keycode, usize>) {
    let mut key_stats: Vec<KeyPress> = key_count.iter()
        .map(|(key, val)| KeyPress {
            key: format!("{:?}", key),
            count: *val,
        }).collect();

    key_stats.sort_by(|a, b| b.count.cmp(&a.count));
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("keypress.json")
        .expect("Unable to open keypress.json");

    let mut writer = BufWriter::new(file);
    for key_val in key_stats {
        writeln!(writer, "{:?}: {}", key_val.key, key_val.count).expect("Unable to write to file");
    }
}

fn run_normal_mode() {
    let device_state = DeviceState::new();
    let mut key_count: HashMap<Keycode, usize>= HashMap::new();

    loop {
        let keys = device_state.get_keys();
        for key in keys {
            *key_count.entry(key).or_insert(0) += 1;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        save_keypress(&key_count);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::load()?;

    if config.run_as_service {
        #[cfg(target_os = "windows")]
        {
            service::run_as_service()?;
        }
        #[cfg(not(target_os = "windows"))]
        {
            println!("Windows服务模式仅支持Windows系统");
            run_normal_mode();
        }
    } else {
        run_normal_mode();
    }

    Ok(())
}
