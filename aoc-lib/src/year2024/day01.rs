// file: src/year2024/day01.rs

// my actual solution code for day 1, year 2024

use crate::utils;
use anyhow::Result;
use std::collections::HashMap;


pub fn solve() -> Result<()> {
    // Define the file path, read the file, and store the content in the input variable
    let input = utils::load_input(2024, 1)?;

    // Solve Part 1
    let result_part1 = solve_part1(&input);
    let result_part2 = solve_part2(&result_part1.1, &result_part1.2);
    println!("Day 1 / Part 1--> Distance is: {}", result_part1.0);
    println!("Day 1 / Part 2--> Similarity is: {}", result_part2);

    Ok(())
}

fn solve_part1(input: &str) -> (i32, Vec<i32>, Vec<i32>) {
    let (mut left_numbers, mut right_numbers) = create_two_lists(input).expect("Invalid input format");

    sort_lists(&mut left_numbers);
    sort_lists(&mut right_numbers);

    let distance = find_distance(&left_numbers, &right_numbers).expect("Failed to calculate distance");
    (distance, left_numbers, right_numbers)
}

fn solve_part2(left_numbers:&Vec<i32>, right_numbers: &Vec<i32>) -> i32 {
    calculate_similarity(left_numbers, right_numbers).expect("Failed to calculate similarity")
}

// BEGIN: Part 1
fn create_two_lists(content: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in content.trim().lines() {
        let mut parts = line.split_whitespace();
        if let (Some(left), Some(right)) = (parts.next(), parts.next()) {
            left_numbers.push(left.parse::<i32>()?);
            right_numbers.push(right.parse::<i32>()?);
        }
    }
    Ok((left_numbers, right_numbers))
}

fn sort_lists(lists: &mut [i32]) {
    lists.sort();
}

fn find_distance(list1: &[i32], list2: &[i32]) -> Result<i32> {
    anyhow::ensure!(list1.len() == list2.len(), "Lists must have the same length!");

    let total_distance: i32 = list1.iter().zip(list2).map(|(a, b)| (a - b).abs()).sum();
    Ok(total_distance)
}
// END: Part 1


// BEGIN: Part 2
fn calculate_similarity(left_numbers: &Vec<i32>, right_numbers: &Vec<i32>) -> Result<i32> {
    // Create a hashmap to count occurrences in right_numbers
    let mut counts = HashMap::new();
    for &num in right_numbers {
        *counts.entry(num).or_insert(0) += 1;
    }

    // Calculate the similarity
    let mut total_similarity: i32 = 0;
    for &num in left_numbers {
        if let Some(&count) = counts.get(&num) {
            // Safe multiplication and addition
            let product = num.checked_mul(count)
                .ok_or_else(|| anyhow::anyhow!("Overflow while calculating similarity"))?;
            total_similarity = total_similarity.checked_add(product)
                .ok_or_else(|| anyhow::anyhow!("Overflow while calculating similarity"))?;
        }
    }

    Ok(total_similarity)
}