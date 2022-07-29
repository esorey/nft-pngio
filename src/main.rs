use image::imageops::overlay;
use image::io::Reader as ImageReader;
use serde::{Deserialize, Serialize};
use std::fs::File;

const IN_FILE: &'static str = "input.json";

#[derive(Serialize, Deserialize)]
struct Asset {
    outfile: String,
    layer_paths: Vec<String>,
}

impl Asset {
    fn build(&self) {
        let mut layers = self
            .layer_paths
            .iter()
            .map(|path| ImageReader::open(path).unwrap().decode().unwrap());
        let mut base = layers.next().unwrap();
        layers.for_each(|layer| overlay(&mut base, &layer, 0i64, 0i64));
        base.save(&self.outfile).unwrap();
    }
}

fn main() {
    let infile = File::open(IN_FILE).unwrap();
    let assets: Vec<Asset> = serde_json::from_reader(infile).unwrap();
    assets.iter().for_each(|a| a.build());
    println!("Done");
}
