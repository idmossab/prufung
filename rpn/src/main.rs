  fn main() {
        let args: Vec<String> = std::env::args().collect();

        rpn(&args[1]);
    }

    fn rpn(s:&str){
        let mut res =Vec::new();
        let op="+-*/%".to_string();

        for ch in s.split_whitespace(){
            if op.contains(ch){
                if res.len()>=2{
                    let (x,y)=(res.pop().unwrap(),res.pop().unwrap());
                    match ch{
                        "+"=>res.push(y+x),
                        "-"=>res.push(y-x),
                        "*"=>res.push(y*x),
                        "/"=>res.push(y/x),
                        _=>res.push(y%x),
                    }
                }else{
                    println!("Error");
                    return 
                }
            }else{
                if let Ok(num)=ch.parse::<i64>(){
                    res.push(num);
                }else{
                    println!("Error");
                    return 
                }
            }
        }
        if res.len()==1{
            println!("{}",res[0]);
        }else{
            println!("Error");
            return 
        }

    }