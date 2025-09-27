use rot21::*;

fn main() {
    println!("The letter \"a\" becomes: {}", rot21("a"));
    println!("The letter \"m\" becomes: {}", rot21("m"));
    println!("The word \"MISS\" becomes: {}", rot21("MISS"));
    println!("Your cypher will be: {}", rot21("Testing numbers 1 2 3"));
    println!("Your cypher will be: {}", rot21("rot21 works!"));
}

#[test]
fn test_rot21_multiple_cases() {
    assert_eq!("ocdn dn v ozno", rot21("this is a test"));
    assert_eq!("mviyjh ndhkgz rjmyn", rot21("random simple words"));
    assert_eq!(
        "ojj  hpxc    nkvxzn      rjmfn",
        rot21("too  much    spaces      works")
    );
    assert_eq!("mv😋w", rot21("ra😋b"));
    assert_eq!("12Â nãj ábpv", rot21("12Â são água"));

    assert_eq!("VWXY", rot21("ABCD"));
    assert_eq!("GJJFDIB BJJY", rot21("LOOKING GOOD"));
    assert_eq!("WTZ", rot21("BYE"));
}