#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}

pub fn prime_checker(nb: u32) ->  Option<Result<u32, PrimeErr>> {
    if nb <2{
        return None
    }
    if nb ==2 {
        return Some(Ok(2))
    }
    if nb %2==0{
        return Some(Err(PrimeErr::Even))
    }
    let mut i=3;
    while i*i<=nb{
        if nb%i==0{
            return Some(Err(PrimeErr::Divider(i)))
        }
        i+=2;
    }
    Some(Ok(nb))
}