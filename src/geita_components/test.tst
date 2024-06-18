
{
	camera: Camera2D::new(),
	player: Player::new(
		size: i32,
		position: i32,
		texture: Texture::load(Path)
	),
}


pub struct Camera2D;

pub strcut Player {
	pub size: 10i32,
	pub position: i32,
	pub texture: Texture,
}

pub enum Types {
	Camera2D,
	Player
}

pub struct Scene2D {
	objects: Vec<Types>
}

impl Scene2D {
	pub fn spawn(args: T) {
		
		
	}
}

pub sturct projectStruct {
	pub camera: Camera2D,
	pub player: Player,
}

let mut pj = projectStruct {
	...
	...
	..
}

Scene2D::init()

Scene2Dspawn.spawn<projectStruct>(	
	pj
)


