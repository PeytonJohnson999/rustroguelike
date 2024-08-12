use quicksilver::prelude::*;

struct Game;

fn main() {
    let settings = Settings {
        ..Default::default()
    };
    run::<Game>("Quicksilver Roguelike", Vector::new(800, 600), settings);
}

impl State for Game{
    /// Load the assets and initiliaze the game
    fn new() -> Result<Self>{
        Ok(Self)
    }

    /// Process keyboard and mouse, update the game state
    fn update(&mut self, window: &mut Window) -> Result<()>{
        Ok(())
    }

    /// Draw stuff on the screen
    fn draw(&mut self, window: &mut Window) -> Result<()>{
        Ok(())
    }
}
