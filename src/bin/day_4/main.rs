use std::fs;

use regex::Regex;

fn main() {
    println!("\n=== Day 4  ====");

    let inputs =
        fs::read_to_string("src/bin/day_4/input.txt").expect("Unable to find 'input.txt' !");
    // "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n\n".to_string();

    let reg = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)").expect("Failed to create Regex struct");
    let mut contains_total: i32 = 0;
    let mut overlap_total: i32 = 0;

    for line in inputs.lines() {
        if line.is_empty() {
            continue;
        }

        let capts = reg.captures(line).unwrap();

        let elf1 = ElfRange {
            from: capts[1].parse().unwrap(),
            to: capts[2].parse().unwrap(),
        };
        let elf2 = ElfRange {
            from: capts[3].parse().unwrap(),
            to: capts[4].parse().unwrap(),
        };

        /* ===== FRIST PART =====  */
        if fully_contains(&elf1, &elf2) {
            contains_total += 1;
        }

        /* ===== SECOND PART =====  */
        if overlap(&elf1, &elf2) {
            overlap_total += 1;
        }
    }

    println!("\nPart one answer: {}", contains_total);
    println!("\nPart two answer: {}", overlap_total);
}

fn fully_contains(a: &ElfRange, b: &ElfRange) -> bool {
    return (a.from <= b.from && a.to >= b.to) || (b.from <= a.from && b.to >= a.to);
}

fn overlap(a: &ElfRange, b: &ElfRange) -> bool {
    return (a.to <= b.to && a.to >= b.from) || (b.to <= a.to && b.to >= a.from);
}

struct ElfRange {
    from: i8,
    to: i8,
}
