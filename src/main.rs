use std::{fs::File, path::Path};

fn read_audio_file(file_path: &Path) {
    println!("test");
}

fn main() {
    let file_path = Path::new("data/amibient-wind.wav");
    read_audio_file(file_path);
}
