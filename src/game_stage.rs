pub struct GameStage {
    current_frame: usize
}

impl GameStage {
    pub fn new() -> Self {
        GameStage {
            current_frame: 0,
        }
    }

    pub fn start(&mut self) {
        // do your setting of palette as well as all initialization here
    }

    pub fn update(&mut self) {
        // do your update here
        self.current_frame += 1;
    }

    pub fn render(&mut self) {
        // render things here
    }
}