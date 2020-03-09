use std::thread::sleep;
use std::time::Duration;

mod sound_thread;
mod vim;

fn main() {
    //vim::start();

    let st = sound_thread::SoundThread::new();

    st.add_sound(80);

    sleep(Duration::from_secs(2));
    st.add_sound(220);

    sleep(Duration::from_secs(2));
}
