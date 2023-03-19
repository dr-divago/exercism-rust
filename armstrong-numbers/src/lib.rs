pub fn is_armstrong_number(num: u64) -> bool {
    let digits = get_digit(num);
    num == digits.iter()
            .map(|digit| digit.pow(digits.len() as u32))
            .sum::<u64>() 
}

fn get_digit(num: u64) -> Vec<u64> {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect()
}
