pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut res =Vec::new();

    for nb in 1..=i{
        let space=(" ").repeat(nb as usize);
        let ch =v.repeat(nb as usize);
        res.push(format!("{}{}",space,ch));
    }

    for nb in (1..i).rev(){
        let space=(" ").repeat(nb as usize);
        let ch =v.repeat(nb as usize);
        res.push(format!("{}{}",space,ch));
    }
    res
}