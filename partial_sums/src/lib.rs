pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut res=Vec::new();
    let mut a=arr.to_vec();

    for _ in 0..=a.len(){
        res.push(a.iter().sum());
        a.pop();
    }
    res
}