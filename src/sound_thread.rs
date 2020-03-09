use std::sync::mpsc;
use std::thread;
use std::sync::mpsc::Sender;
use rodio::Sink;

pub struct SoundThread {
    send: Sender<Sink>,
}

impl SoundThread {
    pub fn new() -> Self {
        let (send, recv) = mpsc::channel();

        thread::spawn(move|| {
            let mut sink_l: Vec<Sink> = Vec::new();

            loop {
                if let Ok(s) = recv.try_recv() {
                    println!("Appending sound");
                    sink_l.push(s);
                }

                for sink in sink_l.iter() {
                    sink.play();
                }
            }
        });

        SoundThread {
            send,
        }
    }

    pub fn add_sound(&self, freq: u32) {
        let device = rodio::default_output_device().unwrap();

        let s = rodio::source::SineWave::new(freq);

        let sink = Sink::new(&device);
        sink.set_volume(0.02);
        sink.append(s);

        self.send.send(sink);
    }
}