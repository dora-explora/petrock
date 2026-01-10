use std::sync::mpsc::{Receiver, RecvError};
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, Sink, OutputStreamBuilder, Source};

pub enum AudioUpdate {
    Volume(f32),
    Pet(),
    Stop(),
}

pub fn run_audio(receiver: Receiver<AudioUpdate>) {
    let mut volume = 1.;
    let mut output_stream = OutputStreamBuilder::open_default_stream().expect("Could not open default audio stream");
    output_stream.log_on_drop(false);
    let music_source = Decoder::try_from(BufReader::new(File::open("./sounds/music.mp3").unwrap())).unwrap().repeat_infinite();
    let pop_source = Decoder::try_from(BufReader::new(File::open("./sounds/pop.mp3").unwrap())).unwrap().buffered();
    let mixer = output_stream.mixer();
    let music_sink = Sink::connect_new(mixer);
    music_sink.append(music_source);
    let sfx_sink = Sink::connect_new(mixer);
    loop {
        match receiver.recv() {
            Ok(AudioUpdate::Volume(vol)) => { volume = vol; music_sink.set_volume(volume); sfx_sink.set_volume(volume); },
            Ok(AudioUpdate::Pet()) => { if sfx_sink.len() > 2 { sfx_sink.clear(); }; sfx_sink.append(pop_source.clone()); },
            Ok(AudioUpdate::Stop()) => { music_sink.detach(); return; },
            Err(RecvError) => panic!("audio update reciever error"),
        }
    }
}
