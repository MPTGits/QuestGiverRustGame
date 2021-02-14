use ggez::{event, ContextBuilder, graphics};
use ggez::graphics::{DrawParam};
use QuestGiverRustGame::MyGame;
use std::path::PathBuf;
use ggez::conf;
mod enteties;

fn main() {
    //Make a Context.
    let resource_dir = PathBuf::from("./assets");
    let cb = ContextBuilder::new("Quest Giver", "Martin Todorov")
    	.window_setup(conf::WindowSetup::default().title("Quest Giver"))
        .window_mode(conf::WindowMode::default().dimensions(640.0, 480.0))
        .add_resource_path(resource_dir);

    let (mut context, mut event_loop) = cb
        .build()
        .expect("Could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut context);

    // let mut player = enteties::player::Player::new(&mut context, "/home/martogod/RustProgramms/QuestGiverRustGame/assets/King/King_Idle_1.png");

    // Run!
    event::run(&mut context, &mut event_loop, &mut my_game);
}