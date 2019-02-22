fn pig_latin(word:String) -> String {
    let vowels:String = "aeiouAEIOU".to_string();
    let mut start:String = String::new();
    for c in word.chars() {
        if vowels.contains(c) { break; }
        if (c == 'y' || c == 'Y') && start.len() > 0 { break; }
        else { start.push(c); }
    }
    if start.len() > 0 { word[start.len()..].to_string() + &start + &"ay" }
    else { word + "yay" }
}

fn main() {
    println!("howdy => {}", pig_latin("howdy".to_string()));
    println!("yay => {}", pig_latin("yay".to_string()));
    println!("gym => {}", pig_latin("gym".to_string()));
    println!("act => {}", pig_latin("act".to_string()));
}
