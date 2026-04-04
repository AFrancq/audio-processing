use std::{
    fs::File,
    io::{Error, Read},
    path::Path,
};

struct WavHeader {
    riff: [u8; 4],
    size: u32,
    file_type: [u8; 4],
    fmt_marker: [u8; 4],
    subchunk1_size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
    data_marker: [u8; 4],
    subchunk2_size: u32,
}

struct WavData {
    data: Vec<u8>,
}

struct Wav {
    header: WavHeader,
    data: WavData,
}

fn read_wav_file(file: File) -> Result<Wav, Error> {}

fn read_audio_file(file_path: &Path) -> Result<Vec<u8>, Error> {
    let mut data: Vec<u8> = vec![];
    File::open(file_path)?.read_to_end(&mut data)?;
    Ok(data)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("./data/ambient-wind.wav");
    let data: Vec<u8> = read_audio_file(file_path)?;
    print!("Test");
    Ok(())
}
