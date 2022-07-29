use image::imageops::overlay;
use image::io::Reader as ImageReader;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;

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
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("expects one command-line argument: infile");
        std::process::exit(1);
    }
    let infile = File::open(&args[1]).unwrap();
    let assets: Vec<Asset> = serde_json::from_reader(infile).unwrap();
    assets.par_iter().for_each(|a| {
        a.build();
        println!("Wrote {}..", a.outfile)
    });
    println!("Done!");
}
