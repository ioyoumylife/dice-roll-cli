mod lib;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    for arg in args {
        if arg == String::from("--help") {
            println!("working on it.");
            return ();
        }
    }
    println!("To roll dice enter 'r' to quit enter 'q'");
    while true {
        let choice = lib::get_input();
        if &choice == "r" {
            print!("Enter your die roll (i.e. 3d6 + 2d10 + 10): ");
            let user_roll = lib::get_input();
            if &user_roll == "q" {
                break;
            } else {
                match lib::DiceRoll::from_str(&user_roll) {
                    Ok(d) => match d.roll() {
                        Ok(roll) => println!("\nResult: {}\n{}\n", roll.0, roll.1),
                        Err(e) => println!("{}", e),
                    },
                    Err(e) => println!("{}", e),
                }
            }
        } else if &choice == "q" {
            println!("Thanks for rolling!");
            break;
        } else {
            println!("Invlaid input, please either (r)oll or (q)uit");
        }
        print!("Enter your die roll (i.e. 3d6 + 2d10 + 10): ");
    }
}
