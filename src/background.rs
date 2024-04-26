use std::io::Cursor;
use std::str::FromStr;
use rodio::{Decoder, Source};
use std::time::Duration;
use rodio::source::Zero;
use crate::rhythm::Rhythm;

pub struct RodioPlayer {
    bpm: u64,
    beats_per_measure: usize,
    rhythm: Rhythm,
    stream: rodio::OutputStream,
    stream_handle: rodio::OutputStreamHandle,
    sink: rodio::Sink,
    up: Vec<u8>,
    down: Vec<u8>
}

impl RodioPlayer {
    const METRONOME_TIME: u64 = 33;

    pub fn new(up: Vec<u8>, down: Vec<u8>) -> Self {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

        let sink = rodio::Sink::try_new(&stream_handle).unwrap();

        Self {
            beats_per_measure: 4,
            bpm: 120,
            rhythm: Rhythm::Quarter,
            sink,
            stream,
            stream_handle,
            up,
            down
        }
    }

    pub fn play(&mut self) {
        self.sink.play()
    }

    pub fn pause(&mut self) {
        self.sink.pause()
    }

    pub fn start(&mut self) {
        // let file = std::fs::File::open("assets/song.mp3").unwrap();
        // let bytes = include_bytes!("../assets/down.wav");
        let my_slice= std::io::Cursor::new(self.up.clone());
        let source = Decoder::new(my_slice)
            .unwrap()
            // .take_duration(Duration::from_millis(Self::METRONOME_TIME))
            .buffered();
        let length = source.total_duration().unwrap().as_millis();
        let beats_per_measure = self.beats_per_measure;


        // let intervals: Vec<u64> = vec![interval; self.beats_per_measure as usize];
        let intervals = self.rhythm.make_intervals(self.bpm, self.beats_per_measure);
        //TODO extract all this, if we have sixteenth we’re accenting every beat instead of every measure
        // Plus eventually we want to accent whatever we want anyway (or hide some beats, etc)
        let it = intervals.into_iter().enumerate().map(move |(n, i)| {
            let s = source.clone();
            let s = if n % beats_per_measure == 0 {
                s.amplify(4.0)
            } else {
                s.amplify(1.0)
            };
            //TODO extract silence?
            let silence: Zero<i16> = Zero::new(source.channels(), source.sample_rate());
            let r = s.mix(silence);
            r.take_duration(Duration::from_nanos(i))
        });
        self.sink
            .append(rodio::source::from_iter(it).repeat_infinite());

        // // self.stream_handle.play_raw(rodio::Decoder::new(BufReader::new(file)).unwrap()).unwrap();
        // let silence_dec = Decoder::new(BufReader::new(file)).unwrap();
        // let silence: Zero<i16> = Zero::new(silence/*/*/**/*/*/_dec.channels(), silence_dec.sample_rate());
        // let decoder_padded = silence_dec.mix(silence);

        // self.sink.append(source);
        // // self.stream_handle.play_raw(rodio::Decoder::new(BufReader::new(file)).unwrap().convert_samples()).unwrap();
        // self.sink.sleep_until_end();
        // thread::sleep(Duration::from_secs(1000));
    }

    pub fn stop(&mut self) {
        self.sink.stop();
    }

    pub fn set_bpm(&mut self, bpm: u64) {
        self.bpm = bpm;
    }
    pub fn set_beats_per_measure(&mut self, beats_per_measure: usize) {
        self.beats_per_measure = beats_per_measure;
    }

    pub fn bpm(&self) -> u64 {
        self.bpm
    }
    pub fn beats_per_measure(&self) -> usize {
        self.beats_per_measure
    }

    pub fn rhythm(&self) -> &Rhythm {
        &self.rhythm
    }

    pub fn set_rhythm(&mut self, rhythm: Rhythm) {
        self.rhythm = rhythm;
    }
}
