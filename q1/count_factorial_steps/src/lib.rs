pub fn count_factorial_steps(factorial: u64) -> u64 {
    if factorial <2{
        return 0
    }
    let mut count =0;
    let mut fact =1;
    while factorial>fact{
        count+=1;
        fact*=count;
    }
    if fact == factorial{
        return count;
    }
   0
}
