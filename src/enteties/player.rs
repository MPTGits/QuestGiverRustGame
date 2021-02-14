use ggez::{graphics, Context};

pub struct Player {
    pub image: graphics::Image,
}

impl Player {

	 pub fn new(context: &mut Context, img_url: &str) -> Player {
	 	println!("{:?}",img_url);
	 	let player_img = graphics::Image::new(context, img_url).unwrap();
        Player {image: player_img}
    }

}