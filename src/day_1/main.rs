use std::fs;

fn main() {
    println!("\n=== Day 1  ====");
    println!("Opening intput.txt");

    let mut current: i32 = 0;
    let mut biggest : i32 = 0;
    let inputs = fs::read_to_string("src/day_1/input.txt").expect("Unable to find 'input.txt' !");

    for x in inputs.split('\n'){
        if x.is_empty(){
            if current > biggest {
                biggest = current;
            }

            current = 0;
        }
        else {
            current += x.parse::<i32>().unwrap();
        }
    }

    println!("\nAnswer: {}", biggest);
}
