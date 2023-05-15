use std::fmt;

enum LinkedList {
    None,
    Some(Box<Person>),
}

impl LinkedList {
    fn add(&mut self, person: Box<Person>) {
        match *self {
            LinkedList::None => {
                *self = LinkedList::Some(person);
            },
            LinkedList::Some(ref mut p) => {
                println!("add {}", person.name);
                p.next.add(person);
            }
        }
    }
}

struct Person {
    name: String,
    age: u32,
    next: LinkedList,
}

// implement Disply trait
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name is {}, age is {}", self.name, self.age.to_string())
    }
}

impl Person {
    pub fn new(_name: String, _age: u32) -> Self {
        Person {
            name: _name,
            age: _age,
            next: LinkedList::None,
        }
    }
}

fn main() {
    let mut list = LinkedList::None;
    let p = Person::new("hjongkim".to_string(), 99);
    list.add(Box::new(p));
    
    match list {
        LinkedList::None => {
            println!("list in none");
        },
        LinkedList::Some(ref person) => {
            println!("{}", person);
        }
    }

}