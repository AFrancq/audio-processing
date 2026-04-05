use std::{
    fs::File,
    io::{Error, Read},
    path::Path,
};

trait AudioFile {
    fn read_audio_file(path: &Path) -> Result<Self, Error>
    where
        Self: Sized;
}

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

struct Wav {
    header: WavHeader,
    data: Vec<u8>,
}

impl AudioFile for Wav {
    fn read_audio_file(path: &Path) -> Result<Self, Error> {
        let file = File::open(path)?;
        read_wav(file)
    }
}

fn read_wav(mut file: File) -> Result<Wav, Error> {
    let mut buffer = [0u8; 44];
    file.read_exact(&mut buffer)?;
    let header = WavHeader {
        riff: [buffer[0], buffer[1], buffer[2], buffer[3]],
        size: u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
        file_type: [buffer[8], buffer[9], buffer[10], buffer[11]],
        fmt_marker: [buffer[12], buffer[13], buffer[14], buffer[15]],
        subchunk1_size: u32::from_le_bytes([buffer[16], buffer[17], buffer[18], buffer[19]]),
        audio_format: u16::from_le_bytes([buffer[20], buffer[21]]),
        num_channels: u16::from_le_bytes([buffer[22], buffer[23]]),
        sample_rate: u32::from_le_bytes([buffer[24], buffer[25], buffer[26], buffer[27]]),
        byte_rate: u32::from_le_bytes([buffer[28], buffer[29], buffer[30], buffer[31]]),
        block_align: u16::from_le_bytes([buffer[32], buffer[33]]),
        bits_per_sample: u16::from_le_bytes([buffer[34], buffer[35]]),
        data_marker: [buffer[36], buffer[37], buffer[38], buffer[39]],
        subchunk2_size: u32::from_le_bytes([buffer[40], buffer[41], buffer[42], buffer[43]]),
    };
    let mut data = vec![0u8; header.subchunk2_size as usize];
    file.read_exact(&mut data)?;
    Ok(Wav { header, data })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("./data/ambient-wind.wav");
    let wav = Wav::read_audio_file(file_path)?;
    print!("Test");
    Ok(())
}
