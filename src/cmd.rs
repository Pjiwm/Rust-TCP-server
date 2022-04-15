pub fn reverse_string(s: &str) -> String {
    let char_vec: Vec<char> = s.chars().collect();
    let remain_count = s.chars().count() - 1;
    let last_char = char_vec.get(remain_count).unwrap();

    let mut remaining_str: String = "".to_owned();
    for i in 0..remain_count {
        remaining_str.push(*char_vec.get(i).unwrap());
    }
    if remain_count > 0 {
        return format!("{}{}", last_char, reverse_string(remaining_str.as_str()));
    }
    return "\n".to_owned();
}
