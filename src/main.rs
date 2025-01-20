use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use device_query::{DeviceQuery, DeviceState, Keycode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct KeyPress {
    key: String,
    count: usize,
}

fn save_keypress(key_count: &HashMap<Keycode, usize>) {
    let mut key_stats: Vec<KeyPress> = key_count.iter()
        .map(|(key, val)| KeyPress {
            key: format!("{:?}", key),
            count: *val,
        }).collect();

    // 排序按键统计数据，按频率从高到低
    key_stats.sort_by(|a, b| b.count.cmp(&a.count));
    // 将结果写入文件
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

#[tokio::main]
async fn main() {
    let device_state = DeviceState::new();
    let mut key_count: HashMap<Keycode, usize>= HashMap::new();

    loop {
        // 获取当前按下的所有键
        let keys = device_state.get_keys();

        // 记录按键
        for key in keys {
            *key_count.entry(key).or_insert(0) += 1;
        }
        // 每隔1秒钟进行一次统计并存储
        std::thread::sleep(std::time::Duration::from_secs(1));
        // 定期保存按键统计
        save_keypress(&key_count);
    }
}
