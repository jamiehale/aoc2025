use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

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

fn dfs(
    device: &Device,
    visited: &mut HashSet<String>,
    results: &mut Vec<Device>,
    device_map: &HashMap<String, Device>,
) {
    if visited.contains(&device.name) {
        return;
    }
    visited.insert(device.name.clone());

    for neighbour in device.connections.iter() {
        dfs(
            device_map.get(neighbour).unwrap(),
            visited,
            results,
            device_map,
        );
    }

    results.push(device.clone());
}

fn topo_sort(devices: &[Device], device_map: &HashMap<String, Device>) -> Vec<Device> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut results: Vec<Device> = vec![];

    for device in devices.iter() {
        dfs(device, &mut visited, &mut results, device_map);
    }

    results.reverse();
    results
}

#[derive(PartialEq)]
enum Direction {
    Forward,
    Reverse,
}

fn paths_starting_from(
    devices: &[Device],
    from: &str,
    direction: Direction,
) -> HashMap<String, usize> {
    let mut paths_from: HashMap<String, usize> = HashMap::new();

    for device in devices.iter() {
        paths_from.insert(device.name.clone(), if device.name == from { 1 } else { 0 });
    }

    if direction == Direction::Forward {
        for device in devices.iter() {
            let so_far = *paths_from.get(&device.name).unwrap();
            for name in device.connections.iter() {
                if let Some(count) = paths_from.get_mut(name) {
                    *count += so_far;
                }
            }
        }
    } else {
        for device in devices.iter().rev() {
            for name in device.connections.iter() {
                let so_far = *paths_from.get(name).unwrap();
                if let Some(count) = paths_from.get_mut(&device.name) {
                    *count += so_far;
                }
            }
        }
    }

    paths_from
}

fn count_paths(devices: &[Device], from: &str, to: &str) -> usize {
    let paths_from = paths_starting_from(devices, from, Direction::Forward);

    *paths_from.get(to).unwrap()
}

fn main() {
    let mut devices = stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| to_device(&l))
        .collect::<Vec<Device>>();
    devices.push(Device {
        name: String::from("out"),
        connections: vec![],
    });
    let device_map = build_device_map(&devices);
    let topo_sorted = topo_sort(&devices, &device_map);
    let paths_from_start = paths_starting_from(&topo_sorted, "svr", Direction::Forward);
    let paths_to_end = paths_starting_from(&topo_sorted, "out", Direction::Reverse);
    let svr_to_fft = paths_from_start.get("fft").unwrap();
    let fft_to_dac = count_paths(&topo_sorted, "fft", "dac");
    let dac_to_out = paths_to_end.get("dac").unwrap();
    let result = svr_to_fft * fft_to_dac * dac_to_out;
    println!("{}", result);

    // let fft_to_dac = count_paths(&topo_sorted, "fft", "dac");
    // let dac_to_fft = count_paths(&topo_sorted, "dac", "fft");
    // println!("fft -> dac: {}", fft_to_dac); // 4199340
    // println!("dac -> fft: {}", dac_to_fft); // 0
}
