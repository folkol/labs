use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, Read, Seek, SeekFrom};
use std::thread;

type StatsMap = BTreeMap<String, Stats>;

struct Stats {
    min: f64,
    sum: f64,
    count: usize,
    max: f64,
}

impl Stats {
    fn merge(&mut self, partial: &Stats) {
        self.min = self.min.min(partial.min);
        self.max = self.max.max(partial.max);
        self.sum += partial.sum;
        self.count += partial.count;
    }

    fn process(&mut self, temp: f64) {
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
    // let file_path = "../java-orig/measurements.txt";
    let file_path = "/tmp/measurements_smol.txt";
    let mut stats: BTreeMap<String, Stats> = Default::default();

    let num_shards = thread::available_parallelism().unwrap().get();
    let file = File::open(file_path).unwrap();

    let mut handles = vec![];
    let partitions = find_partitions(file, num_shards);
    for window in partitions.windows(2) {
        let [begin, end] = [window[0], window[1]];
        let handle = thread::spawn(move || {
            let file = File::open(file_path).unwrap();
            handle_shard(&file, begin, end)
        });
        handles.push(handle);
    }

    for handle in handles {
        let partial_stats = handle.join().unwrap();
        merge(&mut stats, partial_stats);
    }

    print_report(&mut stats);
}

fn find_partitions(mut file: File, num_shards: usize) -> Vec<u64> {
    let mut cuts = Vec::new();
    let len = file.metadata().unwrap().len();
    let shard_size = len / num_shards as u64;

    cuts.push(0);
    for i in 1..num_shards {
        let approximate_pos = i as u64 * shard_size;
        file.seek(SeekFrom::Start(approximate_pos)).unwrap();

        let mut reader = std::io::BufReader::new(&file);
        let mut buf = Vec::new();
        reader.read_until(b'\n', &mut buf).unwrap();

        let pos_after_newline = approximate_pos + buf.len() as u64;
        cuts.push(pos_after_newline);
    }
    cuts.push(len);

    cuts
}

fn handle_shard(mut file: &File, begin: u64, end: u64) -> StatsMap {
    file.seek(SeekFrom::Start(begin)).unwrap();
    let reader = std::io::BufReader::new(file.take(end - begin));
    let mut stats: StatsMap = Default::default();
    for line in reader.lines() {
        let line = line.unwrap();
        let (station, temp) = line.split_once(';').unwrap();
        let temp: f64 = temp.parse().unwrap();
        stats.entry(station.to_string()).or_default().process(temp);
    }
    stats
}

fn merge(total: &mut StatsMap, partial: StatsMap) {
    for (station, stats) in &partial {
        total.entry(station.to_string()).or_default().merge(stats);
    }
}

fn print_report(stats: &mut BTreeMap<String, Stats>) {
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
