use std::fs::File;
use std::io::Read;

pub fn read(day: &str) -> String {
    let mut file = match File::open(format!("./inp/day{}.txt", {day})) {
        Ok(file) => file,
        Err(_) => return String::new(), 
    };
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return String::new(); 
    }
    contents
}
