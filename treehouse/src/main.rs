use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote {note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, greeting: &str,action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("welcome to the tree house, {}",self.name),
            VisitorAction::AcceptWithNote {note} => {
                println!("welcome to the treehouse, {}",self.name);
                println!("{}",note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", "Hello Bert, enyoy",VisitorAction::Accept,45),
        Visitor::new("steve", "hola steve bro",VisitorAction::AcceptWithNote{note: String::from("lactoce free milk")},15),
        Visitor::new("fred", "que tal",VisitorAction::Refuse,30),
    ];
    loop {
    println!("hello wat name (Leave empty and press ENTER to quit)");

    let name = what_is_your_name();
    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => {
            if name.is_empty() {
                break;
            } else {
                println!("{} is not on the visitor list", name);
                visitor_list.push(Visitor::new(&name, "New friend",VisitorAction::Probation,0));
            }
        }
    }
}
    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}
