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
    EditBill,
    RemoveBill,
}
impl MainMenu {
    fn show() {
        println!("");
        println!("===Main Menu===");
        println!("1. Add Bill");
        println!("2. Show Bill");
        println!("3. Edit Bill");
        println!("4. remove Bill");
        println!("")
    }
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(MainMenu::AddBill),
            "2" => Some(MainMenu::ShowBill),
            "3" => Some(MainMenu::EditBill),
            "4" => Some(MainMenu::RemoveBill),
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
            println!("name: {}, amount: {}", b.name, b.amount);
        }
    }
    fn edit_bill(bills: &mut Bills) {
        // show bill first
        MainMenu::show_bills(bills);
        println!("Please enter the bill you want to edit");
        let bill_name = match input::get_input() {
            Some(s) => s,
            None => return,
        };
        // find bill by name
        let bill = match bills.inner.get_mut(&bill_name) {
            Some(b) => b,
            None => return,
        };
        println!("Please enter the new amount");
        let new_amount = match input::get_input() {
            Some(s) => match s.parse::<f64>() {
                Ok(n) => n,
                Err(_e) => return,
            },
            None => return,
        };
        bill.amount = new_amount;
        println!("Bill edited successfully");
    }
    fn remove_bill(bills: &mut Bills) {
        MainMenu::show_bills(bills);
        println!("Please enter the bill you want to delete");
        let bill_name = match input::get_input() {
            Some(s) => s,
            None => return,
        };
        bills.inner.remove(&bill_name);
    }
}

#[derive(Debug, Clone)]
struct Bill {
    amount: f64,
    name: String,
}
struct Bills {
    inner: HashMap<String, Bill>,
}
impl Bill {
    fn new(name: String, amount: f64) -> Self {
        Bill { name, amount }
    }
}
impl Bills {
    pub fn new() -> Self {
        Bills {
            inner: HashMap::new(),
        }
    }
    pub fn add_bill(&mut self, bill: Bill) {
        self.inner.insert(bill.name.clone(), bill);
    }
    pub fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }
}

fn run_main() -> Option<String> {
    let mut bills = Bills::new();
    loop {
        MainMenu::show();
        let choice = input::get_input()?;
        match MainMenu::from_str(choice.as_str()) {
            Some(MainMenu::AddBill) => MainMenu::add_bill(&mut bills),
            Some(MainMenu::ShowBill) => MainMenu::show_bills(&bills),
            Some(MainMenu::EditBill) => MainMenu::edit_bill(&mut bills),
            Some(MainMenu::RemoveBill) => MainMenu::remove_bill(&mut bills),
            None => {
                println!("invalid choice");
                break;
            }
        }
    }

    return None;
}

fn main() {
    run_main();
}
