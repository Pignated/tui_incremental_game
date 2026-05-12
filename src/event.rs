use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use color_eyre::eyre::Context;
use crossterm::event::{self, Event as CrosstermEvent};

pub const TPS: f64 = 60.0;
pub enum Event {
    Tick,
    Crossterm(CrosstermEvent),
    App(AppEvent),
}

pub enum AppEvent {
    GoUp,
    GoDown,
    GoLeft,
    GoRight,
    Select,
    Quit,
}
//The main thread holds on to this as a way to listen for events
pub struct EventHandler {
    tx: mpsc::Sender<Event>,
    rx: mpsc::Receiver<Event>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let ev_thread = EventThread::new(tx.clone());
        thread::spawn(|| ev_thread.run());
        Self { tx, rx }
    }
    pub fn send(&self, app_event: AppEvent) {
        let _ = self.tx.send(Event::App(app_event));
    }
    pub fn next(&self) -> color_eyre::Result<Event> {
        Ok(self.rx.recv()?)
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

//Is what we use for the game loop
//This is the listener for inputs and the clock
pub struct EventThread {
    tx: mpsc::Sender<Event>,
}
impl EventThread {
    fn new(tx: mpsc::Sender<Event>) -> Self {
        Self { tx }
    }
    fn run(self) -> color_eyre::Result<()> {
        let tick = Duration::from_secs_f64(1.0 / TPS);
        let mut last_tick = Instant::now();
        loop {
            let elapsed = last_tick.elapsed();
            if elapsed >= tick {
                self.send(Event::Tick);
                last_tick = Instant::now();
                continue;
            }
            let timeout = tick.saturating_sub(elapsed);
            // poll for crossterm events, ensuring that we don't block the tick interval
            if event::poll(timeout).wrap_err("failed to poll for crossterm events")? {
                let event = event::read().wrap_err("failed to read crossterm event")?;
                self.send(Event::Crossterm(event));
            }
        }
    }

    fn send(&self, event: Event) {
        let _ = self.tx.send(event);
    }
}
