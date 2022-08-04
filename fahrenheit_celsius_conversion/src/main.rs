use std::io;

fn main() {
    let temp: f64 = loop {
        println!("Please enter a temperature value.");

        let mut temp = String::new();
        io::stdin().read_line(&mut temp)
            .expect("Failed to read line");
        
        match temp.trim().parse() {
            Ok(num) => {
                break num;
            }
            Err(_) => {
                continue;
            }
        }
    };

    println!("{}°C => {:.1}°F", temp, temp * 1.8 + 32.0);
    println!("{}°F => {:.1}°C", temp, (temp - 32.0) / 1.8);
}
