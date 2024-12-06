pub struct ProgressiveRenderer {
    current_resolution: u32,
    target_resolution: u32,
    is_rendering: bool,
    last_render_time: std::time::Instant,
    min_render_interval: std::time::Duration,
}

impl ProgressiveRenderer {
    pub fn new(target_resolution: u32) -> Self {
        Self {
            current_resolution: 16,  // Commencer avec une résolution très basse
            target_resolution,
            is_rendering: false,
            last_render_time: std::time::Instant::now(),
            min_render_interval: std::time::Duration::from_millis(50),
        }
    }

    pub fn should_render(&self) -> bool {
        // Modifier la condition pour permettre le rendu initial
        self.current_resolution >= self.target_resolution
            || self.last_render_time.elapsed() >= self.min_render_interval
    }

    pub fn next_resolution(&mut self) -> u32 {
        if self.current_resolution > 1 {
            self.current_resolution /= 2;
        }
        self.last_render_time = std::time::Instant::now();
        self.current_resolution
    }

    pub fn reset(&mut self) {
        self.current_resolution = 16;
        self.is_rendering = true;
    }
}
