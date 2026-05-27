pub struct Robot {
    pub id: u32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub battery: i32,
}

pub struct World {
    pub robots: Vec<Robot>,
}

impl World {
    pub fn tick(&mut self) {
        for robot in &mut self.robots {
            robot.battery -= 1;
        }
    }
}

fn main() {
    let mut world = World {
        robots: vec![
            Robot {
                id: 1,
                name: String::from("Atlas"),
                x: 0,
                y: 0,
                battery: 100,
            }
        ],
    };

    for tick in 0..5 {
        println!("Tick {}", tick);
        world.tick();
    }
}