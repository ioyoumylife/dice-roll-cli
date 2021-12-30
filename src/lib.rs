use std::str::FromStr;

use rand::Rng;

pub fn get_input() -> String {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub enum DiceType {
    D2,
    D3,
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

impl DiceType {
    fn from(val: u16) -> Result<DiceType, &'static str> {
        if val == 2 {return Ok(DiceType::D2);}
        else if val == 3 {return Ok(DiceType::D3);}
        else if val == 4 {return Ok(DiceType::D4);}
        else if val == 6 {return Ok(DiceType::D6);}
        else if val == 8 {return Ok(DiceType::D8);}
        else if val == 10 {return Ok(DiceType::D10);}
        else if val == 12 {return Ok(DiceType::D12);}
        else if val == 20 {return Ok(DiceType::D20);}
        else if val == 100 {return Ok(DiceType::D100);}
        else {
            return Err("Error: Could not parse integer into dice type\nAllowed dice are d2, d3, d4, d6, d8, d10, d12, d20, d100");
        }
    }
    fn value(&self) -> u8 {
        match self {
            DiceType::D2 => 2,
            DiceType::D3 => 3,
            DiceType::D4 => 4,
            DiceType::D6 => 6,
            DiceType::D8 => 8,
            DiceType::D10 => 10,
            DiceType::D12 => 12,
            DiceType::D20 => 20,
            DiceType::D100 => 100,
        }
    }
}

pub struct DiceRoll {
    dice: Vec<DiceType>,
    num: Vec<u16>,
    modifiers: i32,
}

impl DiceRoll {
    pub fn from(dice: Vec<DiceType>, num: Vec<u16>, modifiers: i32) -> DiceRoll {
        DiceRoll { dice, num, modifiers }
    }
    pub fn from_str(input: &str) -> Result<DiceRoll, &str> {
        let mut modifiers = 0;
        let mut dice: Vec<DiceType> = vec![];
        let mut num: Vec<u16> = vec![];
        let mut mod_string = match String::from_str(input) {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        mod_string = mod_string.trim().to_string();
        mod_string = mod_string.replace(" ", "").to_string();
        mod_string = mod_string.replace("\t", "").to_string();
        mod_string = mod_string.replace("D", "d").to_string();
        // println!("{:?}", mod_string);
        let my_vec = mod_string.split("+");
        for item in my_vec {
            let item_vec: Vec<&str> = item.split("d").collect();
            if item_vec.len() == 1 {
                let m: i32 = match item_vec[0].parse() {
                    Ok(i) => i,
                    Err(_) => 0,
                };
                modifiers += m;
            } else if item_vec.len() == 2 {
                for (n, num_str) in item_vec.iter().enumerate() {
                    let num_u16: u16 = match num_str.parse() {
                        Ok(n) => n,
                        Err(_) => return Err("Error: Cannot parse string as int."),
                    };
                    if n % 2 == 0 {
                        num.push(num_u16);
                    } else if n % 2 == 1 {
                        let dt = match DiceType::from(num_u16) {
                            Ok(r) => r, 
                            Err(e) => return Err(e),
                        };
                        dice.push(dt);
                    } else {
                        return Err("Error: Invalid syntax");
                    }
                }
            }
        }
        return Ok(DiceRoll::from(dice, num, modifiers));
    }
    fn validate(&self) -> Result<(), &str> {
        if self.dice.len() == self.num.len() {
            return Ok(());
        } else {
            return Err("Error: Dice and numeric vector lengths are not equl.");
        }
    }
    /// Rolls the collection of dice in the DiceRoll struct, and returns a tuple containing the result as an integer and 
    /// as a string with the information about each roll
    /// 
    /// # Examples
    ///  
    /// ```
    /// let my_dice = DiceRoll::from(vec![DiceType::D20], vec![1], 0);
    /// let d20_roll = my_dice.roll();
    /// ```
    pub fn roll(&self) -> Result<(i32, String), &str> {
        let mut result: i32 = 0;
        let mut result_string = String::new();
        let mut roller = rand::thread_rng();
        match self.validate() {
            Ok(_) => {
                for (num, num_dice) in self.num.iter().enumerate() {
                    result_string.push_str(&format!("{}d{}(", num_dice, self.dice[num].value()));
                    for n in 1..=*num_dice {
                        let die_roll = roller.gen_range(1..=self.dice[num].value());
                        result += die_roll as i32;
                        result_string.push_str(&format!("{}", die_roll));
                        if &n != num_dice {
                            result_string.push_str(" + ");
                        }
                    }
                    result_string.push_str(")");
                    if num < self.num.len() - 1 {
                        result_string.push_str(" + ");
                    }
                }
                if &self.modifiers != &0 {
                    result_string.push_str(&format!(" + {}", &self.modifiers));
                }
                Ok((result + self.modifiers, result_string))
            },
            Err(e) => Err(e),
        }
    }
}