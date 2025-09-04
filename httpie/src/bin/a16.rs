trait Ticket {
    fn show(&self);
}

struct Ticket1;
impl Ticket for Ticket1 {
    fn show(&self) {
        println!("show ticket1");
    }
}

struct CenterTicket;
impl Ticket for CenterTicket {
    fn show(&self) {
        println!("show center");
    }
}

struct Ticket2<T>
where
    T: Ticket,
{
    inner: T,
}

impl<T: Ticket> Ticket2<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
    pub fn show(&self) {
        self.inner.show();
    }
}

fn main() {
    let t = Ticket2::new(Ticket1);
    t.show();
    let t = Ticket2::new(CenterTicket);
    t.show();
}
