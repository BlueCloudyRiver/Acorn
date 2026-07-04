use rand;

fn main() {
    let hex = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

    for _i in 0..5 {
        let mut code = String::from('#');
        for _i in 0..6 {
            let num = rand::random_range(0..=15);
            code.push(hex[num]);
        };
        println!("{}", code)
    }
}