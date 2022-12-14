use std::fs;

fn main() {
    println!("\n=== Day 6  ====");

    let inputs =
        fs::read_to_string("src/bin/day_6/input.txt").expect("Unable to find 'input.txt' !");
    // "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw\n".to_string();

    const SIZE_FIRST_MARKER: usize = 4;
    const SIZE_SECOND_MARKER: usize = 14;

    let bytes = inputs.as_bytes();
    let mut first_marker: usize = 0;
    let mut second_marker: usize = 0;

    /* ===== FIRST PART ===== */
    for i in SIZE_FIRST_MARKER..inputs.len() {
        if !contains_duplicate(&bytes[i - SIZE_FIRST_MARKER..i]) {
            first_marker = i;
            break;
        }
    }

    /* ===== SECOND PART ===== */
    for i in SIZE_SECOND_MARKER..inputs.len() {
        if !contains_duplicate(&bytes[i - SIZE_SECOND_MARKER..i]) {
            second_marker = i;
            break;
        }
    }

    println!("\nPart one answer: {}", first_marker);
    println!("\nPart two answer: {}", second_marker);
}

fn contains_duplicate(tab: &[u8]) -> bool {
    for i in 0..(tab.len() - 1) {
        for j in (i + 1)..tab.len() {
            if tab[i] == tab[j] {
                return true;
            }
        }
    }

    return false;
}
