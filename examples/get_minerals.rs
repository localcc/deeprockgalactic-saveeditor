extern crate deeprockgalactic_saveeditor;
use std::{fs::File, io::Read, path::Path};

use deeprockgalactic_saveeditor::deep_rock_galactic::{SaveFile};

fn main() {
    let mut buf = Vec::new();

    let path = Path::new("save.sav");
    let mut file = File::open(&path).expect("Failed to open save file!");
    file.read_to_end(&mut buf).expect("Failed to read save file!");

    let guids_path = Path::new("guids.json");
    let mut guids_file = File::open(&guids_path).expect("Failed to open guids!");
    let mut guids = String::new();
    guids_file.read_to_string(&mut guids).expect("Failed to read guids!");

    let save_file = SaveFile::new(&mut buf, &guids).expect("Failed to parse save file!");
    println!("Minerals: ");
    println!("\tBismor: {}", save_file.minerals.bismor);
    println!("\tEnor: {}", save_file.minerals.enor);
    println!("\tJadiz: {}", save_file.minerals.jadiz);
    println!("\tCroppa: {}", save_file.minerals.croppa);
    println!("\tMagnite: {}", save_file.minerals.magnite);
    println!("\tUmanite: {}", save_file.minerals.umanite);
}