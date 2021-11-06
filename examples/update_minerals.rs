extern crate deeprockgalactic_saveeditor;
use std::{fs::File, io::{self, Read}, path::Path};

use deeprockgalactic_saveeditor::deep_rock_galactic::{SaveFile};

fn print_minerals(save_file: &SaveFile) {
    println!("\tBismor: {}", save_file.minerals.bismor);
    println!("\tEnor: {}", save_file.minerals.enor);
    println!("\tJadiz: {}", save_file.minerals.jadiz);
    println!("\tCroppa: {}", save_file.minerals.croppa);
    println!("\tMagnite: {}", save_file.minerals.magnite);
    println!("\tUmanite: {}", save_file.minerals.umanite);
}

fn main() {
    let mut buf = Vec::new();

    let path = Path::new("save.sav");
    let mut file = File::open(&path).expect("Failed to open save file!");
    file.read_to_end(&mut buf).expect("Failed to read save file!");

    let guids_path = Path::new("matrix_cores.json");
    let mut guids_file = File::open(&guids_path).expect("Failed to open guids!");
    let mut guids = String::new();
    guids_file.read_to_string(&mut guids).expect("Failed to read guids!");

    let mut save_file = SaveFile::new(&mut buf, &guids).expect("Failed to parse save file!");

    println!("Minerals before modification: ");
    print_minerals(&save_file);
    
    save_file.minerals.bismor += 15f32;
    save_file.minerals.croppa += 20f32;

    println!("Minerals after modification: ");
    print_minerals(&save_file);

    let mut filename = String::new();

    println!("Name for file: ");
    io::stdin().read_line(&mut filename).expect("Failed to read filename!");

    let modified_path = Path::new(&filename);
    let mut modified_file = File::create(&modified_path).expect("Failed to create modified file!");
    save_file.save(&mut modified_file).expect("Failed to write modified file!");
    println!("Modified file written to: {}", filename);
}