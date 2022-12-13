use std::fs;

fn main() {
    println!("\n=== Day 1  ====");
    println!("Opening intput.txt");

    let mut biggests: [i32; 4] = [0, 0, 0, 0];

    let inputs = fs::read_to_string("src/day_1/input.txt").expect("Unable to find 'input.txt' !");

    for x in inputs.split('\n'){
        if x.is_empty(){
            biggests.sort();
            biggests.reverse();

            biggests[3] = 0;
        }
        else {
            biggests[3] += x.parse::<i32>().unwrap();
        }
    }

    println!("\nPart one answer: {}", biggests[0]);
    println!("\nPart two answer: {}", biggests[0] + biggests[1] + biggests[2]);
}
