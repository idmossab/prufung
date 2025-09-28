#[derive(Debug, Clone)]
pub struct Queue {
    pub node: Link,
    pub v:Vec<Person>
}

pub type Link =Option<Box<Person>>;

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub discount: i32,
    pub next_person: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
             node: None,
             v:Vec::new()
        }
    }

   pub fn hilper(&mut self){
        let mut link :Link=None;
        for per in self.v.iter().rev(){
            link=Some(Box::new(
                Person {
                     name: per.name.clone(),
                     discount: per.discount,
                     next_person: link,
                }   
            ));
        }
        self.node=link
   }

    pub fn add(&mut self, name: String, discount: i32) {
        self.v.insert(0, Person {
            name,
            discount,
            next_person: None,
        });
        self.hilper();
    }

    pub fn invert_queue(&mut self) {
        self.v.reverse();
        self.hilper();
    }

    pub fn rm(&mut self) -> Option<(String, i32)> {
        if let Some(p)=self.v.pop(){
            self.hilper();
            return Some((p.name,p.discount));
        }
        None
    }

    pub fn search(&self, name: &str) -> Option<(&String, &i32)> {
        for per in &self.v{
            if per.name ==name{
                return Some((&per.name,&per.discount))
            }
        }
        None
    }
}