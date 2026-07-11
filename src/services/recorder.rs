use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::WavSpec;
use std::sync::{Arc, Mutex};

pub struct Recorder {
    stream: Option<cpal::Stream>,
    writer: Arc<Mutex<Option<hound::WavWriter<std::io::BufWriter<std::fs::File>>>>>,
}

impl Recorder {
    pub fn new() -> Self {
        Recorder {
            stream: None,
            writer: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start_recording(&mut self, filename: &str) -> anyhow::Result<()> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow::anyhow!("Failed to get default input device"))?;
            
        println!("Using default input device");
        
        let config = device.default_input_config()?;
        let sample_format = config.sample_format();
        
        let spec = WavSpec {
            channels: config.channels() as u16,
            sample_rate: config.sample_rate() as u32,
            bits_per_sample: (sample_format.sample_size() * 8) as u16,
            sample_format: if sample_format.is_float() {
                hound::SampleFormat::Float
            } else {
                hound::SampleFormat::Int
            },
        };

        let writer = hound::WavWriter::create(filename, spec)?;
        *self.writer.lock().unwrap() = Some(writer);

        let writer_clone = self.writer.clone();

        let stream_config: cpal::StreamConfig = config.into();

        let stream = match sample_format {
            cpal::SampleFormat::F32 => device.build_input_stream(
                stream_config.clone(),
                move |data: &[f32], _: &_| {
                    if let Some(writer) = writer_clone.lock().unwrap().as_mut() {
                        for &sample in data {
                            writer.write_sample(sample).ok();
                        }
                    }
                },
                move |err| { eprintln!("an error occurred on stream: {}", err); },
                None,
            )?,
            cpal::SampleFormat::I16 => device.build_input_stream(
                stream_config.clone(),
                move |data: &[i16], _: &_| {
                    if let Some(writer) = writer_clone.lock().unwrap().as_mut() {
                        for &sample in data {
                            writer.write_sample(sample).ok();
                        }
                    }
                },
                move |err| { eprintln!("an error occurred on stream: {}", err); },
                None,
            )?,
            cpal::SampleFormat::U16 => device.build_input_stream(
                stream_config.clone(),
                move |data: &[u16], _: &_| {
                    if let Some(writer) = writer_clone.lock().unwrap().as_mut() {
                        for &sample in data {
                            // hound does not directly support u16 if sample_format is Int without conversions in some cases,
                            // but write_sample takes it
                            writer.write_sample(sample as i16).ok();
                        }
                    }
                },
                move |err| { eprintln!("an error occurred on stream: {}", err); },
                None,
            )?,
            _ => return Err(anyhow::anyhow!("Unsupported sample format: {:?}", sample_format)),
        };

        stream.play()?;
        self.stream = Some(stream);

        Ok(())
    }

    pub fn stop_recording(&mut self) -> anyhow::Result<()> {
        if let Some(stream) = self.stream.take() {
            stream.pause()?; // Dropping it also stops it.
        }
        if let Some(writer) = self.writer.lock().unwrap().take() {
            writer.finalize()?;
        }
        Ok(())
    }
}
