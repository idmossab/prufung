pub fn reverse_it(v: i32) -> String {
    let mut res=String::new();
    let abs=(v as i64).abs().to_string();
    let rev :String=abs.chars().rev().collect();
    if v <0{
        res.push('-');
    }
    res.push_str(&rev);
    res.push_str(&abs);
    res
}