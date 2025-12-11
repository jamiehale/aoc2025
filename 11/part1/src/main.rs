use std::{collections::HashMap, io::stdin};

#[derive(Debug, Clone)]
struct Device {
    name: String,
    connections: Vec<String>,
}

fn to_device(line: &str) -> Device {
    let tokens = line
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let name_token = tokens.first().unwrap();
    Device {
        name: name_token[..(name_token.len() - 1)].to_string(),
        connections: tokens.into_iter().skip(1).collect(),
    }
}

fn build_device_map(devices: &[Device]) -> HashMap<String, Device> {
    let mut map: HashMap<String, Device> = HashMap::new();
    for device in devices.iter() {
        map.insert(device.name.clone(), device.clone());
    }
    map
}

fn count_paths_from(device: &Device, device_map: &HashMap<String, Device>) -> usize {
    device.connections.iter().fold(0_usize, |acc, name| {
        if name == "out" {
            acc + 1
        } else {
            acc + count_paths_from(device_map.get(name).unwrap(), device_map)
        }
    })
}

fn main() {
    let devices = stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| to_device(&l))
        .collect::<Vec<Device>>();
    let device_map = build_device_map(&devices);
    let me = device_map.get("you").unwrap();
    let count = count_paths_from(me, &device_map);
    println!("{}", count);
}
