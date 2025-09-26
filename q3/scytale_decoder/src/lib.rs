pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
    let a=letters_per_turn as usize;
    if s.is_empty() || letters_per_turn==0{
        return None
    }
    let mut res=vec![String::new();a];

    for (i,ch) in s.chars().enumerate(){
        res[i%a].push(ch);
    }
    Some(res.concat())
}