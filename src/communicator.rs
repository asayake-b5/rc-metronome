use crate::background::RodioPlayer;
use std::sync::mpsc::Receiver;
use crate::rhythm::Rhythm;

pub enum Msg {
    Play,
    Pause,
    Start,
    Stop,
    SetBpm(u64),
    SetRhythm(Rhythm),
}

pub struct Communicator {
    pub rx: Receiver<Msg>,
    pub player: RodioPlayer,
}

impl Communicator {
    pub fn run(&mut self) {
        loop {
            if let Ok(msg) = self.rx.recv() {
                match msg {
                    Msg::Play => self.player.play(),
                    Msg::Pause => self.player.pause(),
                    Msg::Start => {
                        self.player.stop();
                        self.player.start()
                    }
                    Msg::Stop => self.player.stop(),
                    Msg::SetBpm(bpm) => {
                        self.player.set_bpm(bpm);
                    }
                    Msg::SetRhythm(rhythm) => {
                        self.player.set_rhythm(rhythm);

                    }
                }
            }
        }
    }
}
