use std::sync::{Arc, Mutex, atomic::AtomicBool};

const MAX_TEMPERATURE: f32 = 75.0;

#[derive(Debug, Default)]
pub struct Temperature(Mutex<f32>);

impl Temperature {
    pub fn get(&self) -> f32 {
        *self.0.lock().unwrap()
    }

    pub fn set(&self, value: f32) {
        *self.0.lock().unwrap() = value
    }
}

#[derive(Default, Debug, Clone)]
pub struct Termometer {
    temperature: Arc<Temperature>,
    state: Arc<AtomicBool>,
}

impl Termometer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn temperature(&self) -> f32 {
        self.temperature.get()
    }

    pub fn set_temperature(&self, value: f32) {
        self.temperature.set(value)
    }

    pub fn start(&self) {
        self.state.store(true, std::sync::atomic::Ordering::SeqCst)
    }

    pub fn stop(&self) {
        self.state.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn is_working(&self) -> bool {
        self.state.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn is_overheated(&self) -> bool {
        self.temperature() > MAX_TEMPERATURE
    }
}

impl Drop for Termometer {
    fn drop(&mut self) {
        println!("Dropping the termometer");
        self.state.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}
