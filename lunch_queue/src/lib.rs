#[derive(Debug, Clone)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub discount: i32,
    pub next_person: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { node: None }
    }

    pub fn add(&mut self, name: String, discount: i32) {
        self.node = Some(Box::new(Person { name, discount, next_person: self.node.take() }));
    }

    pub fn rm(&mut self) -> Option<(String, i32)> {
        let mut curr = self.node.as_mut()?;
        if curr.next_person.is_none() {
            let p = self.node.take().unwrap();
            return Some((p.name, p.discount));
        }
        while curr.next_person.as_mut()?.next_person.is_some() {
            curr = curr.next_person.as_mut().unwrap();
        }
        let last = curr.next_person.take().unwrap();
        Some((last.name, last.discount))
    }

    pub fn search(&self, name: &str) -> Option<(&String, &i32)> {
        let mut curr = self.node.as_ref();
        while let Some(person) = curr {
            if person.name == name {
                return Some((&person.name, &person.discount));
            }
            curr = person.next_person.as_ref();
        }
        None
    }

    pub fn invert_queue(&mut self) {
        let (mut prev, mut curr) = (None, self.node.take());
        while let Some(mut person) = curr {
            curr = person.next_person.take();
            person.next_person = prev;
            prev = Some(person);
        }
        self.node = prev;
    }
}
