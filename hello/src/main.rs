#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote(String),
    Refuse,
    Probation,
}

/*
 这里使用 visitor 更好, visitor 表示一个角色, 与行为更密切.
 而 Person 更关注是什么种类的事物, 与实体更接近.
*/
#[derive(Debug)]
struct Visitor {
    name: String,
    greet: String,
    age: usize,
    action: VisitorAction,
}

impl Visitor {
    fn new(name: &str, greet: &str, age: usize, action: VisitorAction) -> Self {
        Self {
            name: name.into(),
            greet: greet.into(),
            age,
            action,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the tree house, {}", self.name),
            VisitorAction::AcceptWithNote(note) => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}

fn main() {
    let mut visitor_list: Vec<Visitor> = vec![
        Visitor::new("Jason", "Hello.", 23, VisitorAction::Accept),
        Visitor::new(
            "Smith",
            "Hi guy.",
            20,
            VisitorAction::AcceptWithNote("Friend".into()),
        ),
    ];

    loop {
        let name = what_is_your_name();

        if name.is_empty() {
            println!("{:?}", visitor_list);
            break;
        }

        let visitor = visitor_list
            .iter()
            .find(|person| person.name.to_lowercase() == name);

        match visitor {
            Some(v) => {
                v.greet_visitor();
            }
            None => {
                println!("Sorry");
                visitor_list.push(Visitor::new(
                    &name,
                    "new friend",
                    0,
                    VisitorAction::Probation,
                ));
            }
        }
    }
}

fn what_is_your_name() -> String {
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("Can not read from stdin.");
    buf.trim().to_lowercase()
}
