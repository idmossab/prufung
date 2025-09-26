use std::collections::HashMap;

pub fn smallest(h: HashMap<&str, i32>) -> i32 {
    // match h.values().min(){
    //     Some(&v)=>v,
    //     None=>i32::MAX
    // }
    let mut v=&i32::MAX;
    for (_,ch) in h.iter(){
        if ch<v{
            v=ch
        }
    }
    *v
}