pub fn rot21(input: &str) -> String {
   let mut res=String::new();

   for ch in input.chars(){
        if ch.is_ascii_lowercase(){
            let rot= (((ch as u8 -97)+21)%26)+97;
            res.push(rot as char);
        }else if ch.is_ascii_uppercase(){
            let rot =(((ch as u8 -65)+21)%26)+65;
            res.push(rot as char);
        }else{
            res.push(ch);
        }
   }
   res
}