/*pub fn is_numeric(string: &str) -> bool {
    let mut result = true;
    for ch in string.chars() {
        if !ch.is_numeric() {
            result = false;
        }
    }
    result
}*/

pub fn is_numeric(string: &str) -> bool {
    string.parse::<isize>().is_ok()
}

pub fn string_to_usize(string: &str) -> usize {
    if let Ok(num) = string.parse::<usize>() {
        num
    }
    else {0}
}