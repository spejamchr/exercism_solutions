pub fn is_armstrong_number(num: u32) -> bool {
    let string = num.to_string();
    let exp = string.chars().count();
    num == string
        .chars()
        .map(|c| c.to_string().parse::<u32>().unwrap().pow(exp as u32))
        .sum()
}
