use std::time;
use rand;
use std::io;


fn main() {
    loop {
        println!("Enter y to play");
        let mut string = String::new();
        io::stdin()
            .read_line(&mut string)
            .expect("failed to capture enter");

        let choice = string.trim().to_string();

        if choice.contains("y") {
            game()
        } else {
            break
        }
    }
}
    


fn game() {
    println!("Wait");
    let rand_time = rand::random_range(1..3);

    std::thread::sleep(
        time::Duration::from_secs(rand_time)
    );

    println!("Go!");
    let start = time::Instant::now();
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("failed to capture enter");

    let elapsed = start.elapsed();
    println!("Reaction time: {}ms", elapsed.as_millis());
}