use csv;
use std::error::Error;

fn main() {
    if let Err(e) = read_from_file("./User_Data.csv") {
        eprintln!("{}", e);
    }
}

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;

        // A little extra unneeded work

        // let id = record[0].parse::<u32>()?;
        // let gender = record[1].to_string();
        // let age = record[2].parse::<u8>()?;
        // let salary = record[3].parse::<u32>()?;
        // let purchased = record[4].parse::<usize>()?;

        let id = match record[0].parse::<u32>() {
            Ok(value) => value,
            Err(e) => {
                println!("Could not parse: {}", e);
                continue;
            }
        };
        // No match block used. Infallible Error Type: Since this enum has no variant, a value of this type can never actually exist
        let gender: String = record[1].parse::<String>()?;

        let age: u8 = match record[2].parse::<u8>() {
            Ok(val) => val,
            Err(e) => {
                println!("Could not parse: {}", e);
                continue;
            }
        };

        let salary: u32 = match record[3].parse::<u32>() {
            Ok(val) => val,
            Err(e) => {
                println!("Could not parse: {}", e);
                continue;
            }
        };

        let purchased: usize = match record[4].parse::<usize>() {
            Ok(val) => val,
            Err(e) => {
                println!("Could not parse: {}", e);
                continue;
            }
        };

        let user: User = User::new(id, gender, age, salary, purchased);

        println!("{:?}", user);
    }

    Ok(())
}

#[derive(Debug)]
struct User {
    id: u32,
    gender: String,
    age: u8,
    salary: u32,
    purchased: usize,
}

impl User {
    fn new(id: u32, gender: String, age: u8, salary: u32, purchased: usize) -> Self {
        Self {
            id,
            gender,
            age,
            salary,
            purchased,
        }
    }
}
