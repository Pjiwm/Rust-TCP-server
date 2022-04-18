pub fn reverse_string(s: &str) -> String {
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
        return format!(
            "{}{}",
            last_char,
            reverse_string_helper(remaining_str.as_str())
        );
    }
    return "".to_owned();
}

pub fn palindrome(s: &str) -> String {
    // we need to format s with \n because we add that to the reverse_string as well
    // this is done in reverse string to show cleaner output
    let is_palindrome = format!("{}\n", s) == reverse_string(s);
    format!("{}\n", is_palindrome)
}

pub fn scream(s: &str) -> String {
    let str_upper = s.to_uppercase();
    let char_vec: Vec<char> = str_upper.chars().collect();
    let mut scream: String = "".to_owned();
    for c in char_vec.iter() {
        scream.push(*c);
        if *c == 'A' || *c == 'I' || *c == 'U' || *c == 'E' || *c == 'O' || *c == 'Y' {
            scream.push(*c);
            scream.push(*c);
        }
    }
    scream.push_str("!!!\n");
    scream
}

#[cfg(test)]
mod tests {
    use crate::string_algorithms;
    #[test]
    fn reverses_string_works_and_adds_new_line() {
        let original = "123456789";
        let reverse = "987654321\n";
        let reversed_by_fn = string_algorithms::reverse_string(original);
        assert_eq!(reverse, reversed_by_fn);
    }
    #[test]
    fn palindrome_returns_false_when_not() {
        let original = "bird";
        let palindrome_check = string_algorithms::palindrome(original);
        assert_eq!(palindrome_check, String::from("false\n"));
    }

    #[test]
    fn palindrome_returns_string_true_when_palindrome() {
        let original = "legovogel";
        let palindrome_check = string_algorithms::palindrome(original);
        assert_eq!(palindrome_check, String::from("true\n")); 
    }

    #[test]
    fn scream_works() {
        let original = "some-text";
        let scream_check = string_algorithms::scream(original);
        assert_eq!(scream_check, String::from("SOOOMEEE-TEEEXT!!!\n"));
    }

}

