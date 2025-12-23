use std::collections::BTreeMap;
use std::io::BufRead;

struct Stats {
    min: f64,
    sum: f64,
    count: usize,
    max: f64,
}

impl Stats {
    fn update(&mut self, temp: f64) {
        self.min = self.min.min(temp);
        self.max = self.max.max(temp);
        self.sum += temp;
        self.count += 1;
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            min: f64::MAX,
            sum: 0.0,
            count: 0,
            max: f64::MIN,
        }
    }
}

fn main() {
    let file = std::fs::File::open("../java-orig/measurements.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut stats: BTreeMap<String, Stats> = BTreeMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let (station, temp) = line.split_once(';').unwrap();
        let temp: f64 = temp.parse().unwrap();

        stats.entry(station.to_string()).or_default().update(temp);
    }

    print!("{{");
    let num_stations = stats.len();
    for (i, (station, stats)) in stats.iter().enumerate() {
        print!(
            "{station}={:.1}/{:.1}/{:.1}",
            stats.min,
            stats.sum / stats.count as f64,
            stats.max
        );
        if i != num_stations - 1 {
            print!(", ");
        }
    }
    println!("}}");
}
