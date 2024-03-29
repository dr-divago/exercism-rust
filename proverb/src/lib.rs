pub fn build_proverb(list: &[&str]) -> String {
    let mut str = String::new();
    if list.len() > 1 {
        for i in 0..list.len() - 1 {
            str.push_str(&format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]));
        }
    }
    if list.len() > 0 {
        str.push_str(&format!("And all for the want of a {}.", list[0]));
    }
    str
}
