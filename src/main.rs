use csv;
use postgres::{Client, NoTls};
use std::error::Error;

fn main() {
    if let Err(e) = read_from_file("./User_Data.csv") {
        eprintln!("{}", e);
    }
}

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut client = create_client()?;

    for result in reader.records() {
        let record = result?;

        // No match block used. Infallible Error Type: Since this enum has no variant, a value of this type can never actually exist
        let id: String = record[0].parse::<String>()?;

        // No match block used. Infallible Error Type: Since this enum has no variant, a value of this type can never actually exist
        let gender: String = record[1].parse::<String>()?;

        let age: i32 = match record[2].parse::<i32>() {
            Ok(val) => val,
            Err(e) => {
                println!("Could not parse: {}", e);
                continue;
            }
        };

        let salary: i32 = match record[3].parse::<i32>() {
            Ok(val) => val,
            Err(e) => {
                println!("Could not parse: {}", e);
                continue;
            }
        };

        let purchased: i32 = match record[4].parse::<i32>() {
            Ok(val) => val,
            Err(e) => {
                println!("Could not parse: {}", e);
                continue;
            }
        };

        let user: User = User::new(id, gender, age, salary, purchased);

        //println!("{:?}", user);

        add_user(&mut client, &user);
    }

    let new_user = User::new(String::from("789123"), String::from("Male"), 33, 72000, 1);

    add_user(&mut client, &new_user)?;

    Ok(())
}

#[derive(Debug)]
struct User {
    id: String,
    gender: String,
    age: i32,
    salary: i32,
    purchased: i32,
}

impl User {
    fn new(id: String, gender: String, age: i32, salary: i32, purchased: i32) -> User {
        User {
            id,
            gender,
            age,
            salary,
            purchased,
        }
    }
}

fn create_client() -> Result<Client, Box<dyn Error>> {
    let client = Client::connect("postgres://postgres:postgres@localhost:5433/csv_test", NoTls)?;

    Ok(client)
}

/* fn add_user(client: &mut Client, user: &User) -> Result<(), Box<dyn Error>> {
    client.execute(
        "INSERT INTO user_data (id, gender, age, salary, purchased) VALUES ($1, $2, $3, $4, $5);",
        &[
            &user.id,
            &user.gender,
            &user.age,
            &user.salary,
            &user.purchased,
        ],
    )?;

    Ok(())
} */

fn add_user(client: &mut Client, user: &User) -> Result<(), Box<dyn Error>> {
    client.execute(
        "INSERT INTO user_data (id, gender, age, salary, purchased) VALUES ($1, $2, $3, $4, $5);",
        &[
            &user.id,
            &user.gender,
            &user.age,
            &user.salary,
            &user.purchased,
        ],
    )?;

    Ok(())
}
