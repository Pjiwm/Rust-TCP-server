pub fn reverse_string(s: &str) -> String {
    println!("{}",s.chars().nth(0).unwrap());
    return format!(
        "{}{}\n",
        reverse_string_helper(s),
        s.chars().nth(0).unwrap()
    );
}
fn reverse_string_helper(s: &str) -> String {
    let char_vec: Vec<char> = s.chars().collect();
    let remain_count = s.chars().count() - 1;
    let last_char = char_vec.get(remain_count).unwrap();

    let mut remaining_str: String = "".to_owned();
    for i in 0..remain_count {
        remaining_str.push(*char_vec.get(i).unwrap());
    }
    if remain_count > 0 {
        return format!("{}{}", last_char, reverse_string_helper(remaining_str.as_str()));
    }
    return "".to_owned();
}

pub fn palindrome(s: &str) -> String {
    println!("{}", s);
    println!("{}", reverse_string(s));
    let is_palindrome = s == reverse_string(s);
    format!("{}\n", is_palindrome)
}
