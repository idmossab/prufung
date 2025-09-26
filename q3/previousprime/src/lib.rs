pub fn prev_prime(nbr: u64) -> u64  {
    for nb in (2..nbr).rev(){
        if (2..nb).all(|i| nb%i !=0){
            return nb
        }
    }
    0
}