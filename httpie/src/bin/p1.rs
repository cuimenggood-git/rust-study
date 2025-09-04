use std::collections::HashMap;

mod input {
    pub fn get_input() -> Option<String> {
        let mut buffer = String::new();
        while std::io::stdin().read_line(&mut buffer).is_err() {
            println!("Invalid Input");
        }
        let res = buffer.trim().to_owned();
        if &res == "" {
            None
        } else {
            Some(res)
        }
    }
}
enum MainMenu {
    AddBill,
    ShowBill,
}
impl MainMenu {
    fn show() {
        println!("");
        println!("===Main Menu===");
        println!("1. Add Bill");
        println!("2. Show Bill");
        println!("")
    }
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(MainMenu::AddBill),
            "2" => Some(MainMenu::ShowBill),
            _ => None,
        }
    }
    fn add_bill(bills: &mut Bills) {
        println!("please input a bill name: ");
        let bill_name = match input::get_input() {
            Some(s) => s,
            None => return,
        };
        println!("please input a bill amount: ");
        let bill_amount = match input::get_input() {
            Some(s) => match s.parse::<f64>() {
                Ok(n) => n,
                Err(_e) => return,
            },
            None => return,
        };
        bills.add_bill(Bill::new(bill_name, bill_amount));
    }
    fn show_bills(bills: &Bills) {
        for b in bills.get_all() {
            println!("{}, {}", b.name, b.amount);
        }
    }
}
#[derive(Debug, Clone)]
struct Bill {
    amount: f64,
    name: String,
}
struct Bills {
    inner: HashMap<String,Bill>,
}
impl Bill {
    fn new(name: String, amount: f64) -> Self {
        Bill { name, amount }
    }
}
impl Bills {
    pub fn new() -> Self {
        Bills { inner: HashMap::new() }
    }
    pub fn add_bill(&mut self, bill: Bill) {
        self.inner.insert(bill.name.clone(), bill);
    }
    pub fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }
}
fn main() {
    let mut bills = Bills::new();
    println!("What is your name?");
    let name = input::get_input();
    match name {
        Some(name) => println!("Hello, {}!", name),
        None => println!("No Name"),
    }
    loop {
        MainMenu::show();
        let choice = input::get_input().expect("enter a choice");
        match MainMenu::from_str(choice.as_str()) {
            Some(MainMenu::AddBill) => MainMenu::add_bill(&mut bills),
            Some(MainMenu::ShowBill) => MainMenu::show_bills(&bills),
            None => {
                println!("invalid choice");
            }
        }
    }
}
