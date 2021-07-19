#[macro_use]
extern crate vst;
extern crate rand;

use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::Events;
use rand::random;

#[derive(Default)]
struct Whisper {
    notes: u8
}

impl Plugin for Whisper {
    fn get_info(&self) -> Info {
        Info {
            name: "Whisper".to_string(),
            unique_id: 1337,
            inputs: 0,
            outputs: 2,
            category: Category::Synth,
            ..Default::default()
        }
    }

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => {
                    match ev.data[0] { // First element vec note on/off. Second midi note
                        144 => self.notes += 1u8, // Note On
                        128 => self.notes -= 1u8, // Note Off
                        _ => (),
                    }
                },
                _ => (),
            }
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        if self.notes == 0 { return }
        let (_, output_buffer) = buffer.split();
    
        for output_channel in output_buffer.into_iter() {
            for output_sample in output_channel {
                *output_sample = (random::<f32>() - 0.5f32) * 2f32;
            }
        }
    }
}


plugin_main!(Whisper);
