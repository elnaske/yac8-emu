use sdl2::Sdl;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

pub struct SquareWave {
    pub phase_inc: f32,
    pub phase: f32,
    pub volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub fn get_audio_device(
    sdl_context: &Sdl,
    freq: Option<i32>,
    channels: Option<u8>,
    samples: Option<u16>,
) -> Result<AudioDevice<SquareWave>, String> {
    let audio_subsystem = sdl_context.audio()?;
    let audio_spec = AudioSpecDesired {
        freq,
        channels,
        samples,
    };
    let audio_device = audio_subsystem.open_playback(None, &audio_spec, |spec| SquareWave {
        phase_inc: 440.0 / spec.freq as f32,
        phase: 0.0,
        volume: 0.25,
    })?;
    Ok(audio_device)
}
