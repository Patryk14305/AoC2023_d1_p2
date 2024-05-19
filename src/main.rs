//Advent of Code 2023 - Day 1 Part 2
//Patryk Perkiewicz

// To Do:
//Check for each digit and each number index and create it in new vector
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn get_value (string: String) -> u32 {

let words: Vec<(&str, &str)> = vec![
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
        ];

        let mut left_word= "";
        let mut left_number= "";
        let mut left_index: usize = string.chars().count();
        let mut right_word= "";
        let mut right_number = "";
        let mut right_index: usize = 0;
    
        for number in &words {
            
            if left_index > string.find(number.0).unwrap_or(left_index) {
                left_index = string.find(number.0).unwrap_or(left_index);
                left_word=number.0;
                left_number=number.1;
            }           
        }
    let left_string = string.replace(left_word, left_number);
    
        for number in &words {
            if right_index < string.rfind(number.0).unwrap_or(right_index) {
                right_index = string.rfind(number.0).unwrap_or(right_index);
                right_word=number.0;
                right_number=number.1;
            } 
        }
        
        let right_string = string.replace(right_word, right_number);

        let mut left_digits: Vec<u32> = vec![];
        let mut right_digits: Vec<u32> = vec![];
        let left_chars = left_string.as_str().chars();
        let right_chars = right_string.as_str().chars();
        
        for char in left_chars {
            if char.is_ascii_digit(){
                left_digits.push(char.to_digit(10).unwrap());

//  Verify digit check:                
           //     println!("{} is digit", char)
            }
            // else {
            //     println!("{} is not a digit", char)
            // }
        }

        for char in right_chars {
            if char.is_ascii_digit(){
                right_digits.push(char.to_digit(10).unwrap());

//  Verify digit check:                
           //     println!("{} is digit", char)
            }
            // else {
            //     println!("{} is not a digit", char)
            // }
        }

        let value = left_digits.first().unwrap()*10 + right_digits.last().unwrap();
        value

}

fn main() {
    let mut sum:u32 = 0;
    let input = read_lines("input.txt");
    //let input = read_lines("test_input.txt");

    for text_line in input{
        sum += get_value(text_line);
    }

    println!("{}", sum)
}
