use fermium::timer::{SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency};

pub struct AppState {
    prev_ticks: u64,
    cur_ticks: u64,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            prev_ticks: 0,
            cur_ticks: unsafe { SDL_GetPerformanceCounter() },
        }
    }

    pub fn ticks(&self) -> f64 {
        self.cur_ticks as f64 / unsafe { SDL_GetPerformanceFrequency() as f64 }
    }

    pub fn delta(&self) -> f64 {
        (self.cur_ticks - self.prev_ticks) as f64 * 1000.0
            / unsafe { SDL_GetPerformanceFrequency() as f64 }
    }

    pub fn update_ticks(&mut self) {
        self.prev_ticks = self.cur_ticks;
        self.cur_ticks = unsafe { SDL_GetPerformanceCounter() };
    }
}
