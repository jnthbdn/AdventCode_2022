use std::fs;

fn main() {
    println!("\n=== Day 6  ====");

    let inputs =
    fs::read_to_string("src/bin/day_6/input.txt").expect("Unable to find 'input.txt' !");
    // "bvwbjplbgvbhsrlpgdmjqwftvncz\n".to_string();

    let bytes = inputs.as_bytes();
    let mut first_marker: usize = 0;

    for i in 4..inputs.len(){
        if ! contains_duplicate(&bytes[i-4..i]){
            first_marker = i;
            break;
        }
    }

    println!("\nPart one answer: {}", first_marker);
    // println!("\nPart two answer: {}", String::from_iter(stacks_2.iter().map(|x| x.back().unwrap())));
}

fn contains_duplicate(tab: &[u8]) -> bool{
    for i in 0..(tab.len() - 1){
        for j in (i+1)..tab.len(){
            if tab[i] == tab[j] {
                return true;
            }
        }
    }

    return false;
}
