mod lib;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    for arg in args {
        if arg == String::from("--help") {
            println!("working on it.");
            return ();
        }
    }
    println!("Enter your die roll (i.e. 3d6 + 2d10 + 10): ");
    let user_roll = lib::get_input();
    match lib::DiceRoll::from_str(&user_roll) {
        Ok(d) => match d.roll() {
            Ok(roll) => println!("Result: {} = {}", roll.1, roll.0),
            Err(e) => println!("{}", e),
        },
        Err(e) => println!("{}", e),
    }
}
