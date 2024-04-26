extern crate core;

use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Router, ServiceExt};
use axum_macros::debug_handler;
use std::io::BufReader;
use std::str::FromStr;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::{fs, thread};
use std::time::{Duration, Instant};
use thread::spawn;

mod background;
mod communicator;
mod rhythm;

use crate::background::RodioPlayer;
use crate::communicator::{Communicator, Msg};
use serde::Deserialize;
use spin_sleep::SpinSleeper;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use crate::rhythm::Rhythm;

#[derive(Clone)]
struct AppState {
    tx: Sender<Msg>,
}

async fn play(State(state): State<AppState>) {
    state.tx.send(Msg::Play).unwrap();
}

async fn pause(State(state): State<AppState>) {
    state.tx.send(Msg::Pause).unwrap();
}
async fn stop(State(state): State<AppState>) {
    state.tx.send(Msg::Stop).unwrap();
}

async fn start(State(state): State<AppState>) {
    state.tx.send(Msg::Start).unwrap();
}

async fn set_bpm(Path(bpm): Path<u64>, State(state): State<AppState>) -> impl IntoResponse {
    state.tx.send(Msg::SetBpm(bpm)).unwrap();
}

async fn set_rhythm(Path(rhythm): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    Rhythm::from_str(&rhythm).and_then(|r| {
        Ok(state.tx.send(Msg::SetRhythm(r)))

    });
}

#[tokio::main]
async fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<Msg>();

    thread::spawn(|| {
        let up = fs::read("assets/up.wav").unwrap();
        let down = fs::read("assets/up.wav").unwrap();
        let mut player = RodioPlayer::new(up, down);
        let mut communicator = Communicator { rx, player };
        communicator.run()
    });

    let mut state = AppState { tx };

    let routes = Router::new()
        .route("/play", get(play))
        .route("/pause", get(pause))
        .route("/push", get(start))
        .route("/stop", get(stop))
        .route("/bpm/:bpm", post(set_bpm))
        .route("/rhythm/:rhythm", post(set_rhythm))
        // .route("/get/bpm", get(bpm))
        .with_state(state);
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
