use std::io::{self, Write};

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("read failed");
    s
}

fn min_max(nums: &[i32]) -> Option<(i32, i32)> {
    if nums.is_empty() {
        return None;
    }
    let mut min = nums[0];
    let mut max = nums[0];

    let mut i = 1;
    while i < nums.len() {
        let n = nums[i];
        if n < min {
            min = n;
        } else if n > max {
            max = n;
        }
        i += 1;
    }

    Some((min, max))
}

fn main() {
    let line = read_line("Enter integers separated by spaces: ");
    let mut nums: Vec<i32> = Vec::new();

    for part in line.trim().split_whitespace() {
        if let Ok(n) = part.parse::<i32>() {
            nums.push(n);
        } else {
            println!("Skipping invalid number: {}", part);
        }
    }

    let result = min_max(&nums); // borrow as a slice
    if let Some((min, max)) = result {
        println!("min = {}, max = {}", min, max);
    } else {
        println!("No numbers provided.");
    }
}