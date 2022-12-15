fn main() {
    let contents = include_str!("test.txt");
    let valid = "123456789-,";
    let sensor_beacons = contents
        .lines()
        .map(|line| line.chars().filter(|c| valid.contains(c)))
        .collect::<Vec<&str>>();
    eprintln!("sensor_beacons = {:?}", sensor_beacons);
}
