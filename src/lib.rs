use ggez::{graphics, Context, GameResult};
use ggez::event::{self, EventHandler};

pub struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_context: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::WHITE);
        // Draw code here...

        let player_img = graphics::Image::new(context, "/King/King_Idle_1.png").unwrap();

        let drawparams = graphics::DrawParam::new()
                                            .dest(ggez::mint::Point2{x:10.0 ,y:10.0});
        graphics::draw(context, &player_img, drawparams);
        graphics::present(context)
    }
}