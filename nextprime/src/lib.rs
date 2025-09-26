pub fn next_prime(nbr: u64) -> u64 {
    let mut res=nbr;
    while res>2{
        if (2..res).all(|i| res%i!=0){
            return res
        }
        res+=1;
    }
    2
}