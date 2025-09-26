pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.replace(letter,"")
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    let mut res =String::new();
    for ch in s.chars(){
        if ch==letter.to_ascii_lowercase() || ch==letter.to_ascii_uppercase(){
           continue 
        }
        res.push(ch)
    }
    res
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
 let mut res =String::new();
    for ch in s.chars(){
        if ch==letter.to_ascii_lowercase(){
           res.push(ch.to_ascii_uppercase()) 
        }else if ch==letter.to_ascii_uppercase(){
           res.push(ch.to_ascii_lowercase()) 
        }else{
            res.push(ch)
        }
    }
    res
}