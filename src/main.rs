#[derive(Debug)]
enum RobotState {
    Idle,
    Moving, 
    Charging,
}

pub struct Robot {
    pub id: u32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub battery: i32,
    pub state: RobotState,
}

pub struct World {
    pub robots: Vec<Robot>,
}

impl World {
    pub fn tick(&mut self) {
        for robot in &mut self.robots {
            match robot.state {
                RobotState::Charging => {
                    robot.battery += 3;

                    if robot.battery >= 100 {
                        robot.battery = 100;
                        robot.state = RobotState::Idle;
                    }
                }
                RobotState::Moving => {
                    robot.battery -= 2;
                    robot.x += 1;

                    if robot.battery <= 20 {
                        robot.state = RobotState::Charging;
                    }
                }
                RobotState::Idle => {
                    robot.battery -= 1;

                    if robot.battery <= 50 {
                        robot.state = RobotState::Moving;
                    }
                }
            }
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
                state: RobotState::Charging,
            }
        ],
    };

    for tick in 0..5 {
        println!("--- Tick {} ---", tick);

        for robot in &world.robots {
            println!(
                "{} | {:?} | battery: {} | position: ({}, {})", 
                robot.name,
                robot.state,
                robot.battery,
                robot.x,
                robot.y
            );
        }

        world.tick();
    }
}
