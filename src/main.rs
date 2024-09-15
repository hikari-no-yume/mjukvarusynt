use std::f32::consts::TAU;

/// MIDI note/key number.
type NoteNo = u8;

const A4_TUNING: f32 = 440.0;
const A4_NOTE: NoteNo = 69;

const SAMPLE_RATE_USZ: usize = 44100;
const SAMPLE_RATE: f32 = SAMPLE_RATE_USZ as f32;

fn main() {
    //           Freude  schöner Göt-ter-fun-ken Tochter aus E---ly--si--um
    let notes = [66, 66, 67, 69, 69, 67, 66, 64, 62, 62, 64, 66, 66, 64, 64];

    let mut audio = Vec::<f32>::new();
    audio.resize(notes.len() * SAMPLE_RATE_USZ, Default::default());

    let hertz = TAU / SAMPLE_RATE;

    for (chunk, &note) in audio.chunks_mut(SAMPLE_RATE_USZ).zip(notes.iter()) {
        let freq = A4_TUNING * ((note as f32 - A4_NOTE as f32) / 12.0).exp2();
        for (i, sample) in chunk.iter_mut().enumerate() {
            *sample = ((i as f32) * freq * hertz).sin();
        }
    }

    std::fs::write(format!("out-f32-mono-{}-pcm.raw", SAMPLE_RATE), unsafe {
        std::slice::from_raw_parts(
            audio.as_ptr() as *const u8,
            audio.len() * std::mem::size_of_val(&audio[0]),
        )
    });
}
