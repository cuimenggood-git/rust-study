enum Color {
    Red,
    Browrn,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("{:?}", "Red"),
            Color::Browrn => println!("{:?}", "Browrn"),
        }
    }
}

struct BoxSize {
    width: f64,
    height: f64,
    depth: f64,
}

impl BoxSize {
    fn new(width: f64, height: f64, depth: f64) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    fn print(&self) {
        println!("width={:?}", self.width);
        println!("height={:?}", self.height);
        println!("depth={:?}", self.depth);
    }
}

struct ShappingBox {
    color: Color,
    box_size: BoxSize,
}

impl ShappingBox {
    fn new(color: Color, box_size: BoxSize) -> Self {
        Self { color, box_size }
    }

    fn print(&self) {
        self.color.print();
        self.box_size.print();
    }
}

#[derive(Debug)]
enum PromtoType {
    PromtoA,
    PromtoB(i32),
}

fn print_promto(p: PromtoType) {
    match p {
        PromtoType::PromtoA => {
            println!("promto a")
        }

        PromtoType::PromtoB(amount) => {
            println!("promto b {}", amount)
        }
    }
}

fn main() {
    let _ = Color::Red;
    let sb = ShappingBox::new(Color::Browrn, BoxSize::new(12.1, 22.2, 33.6));
    sb.print();

    let mut v: Vec<i64> = vec![1, 2, 3];
    v.push(4);

    for item in &v {
        // for item in v { // error[E0507]: cannot move out of `v` because it is borrowed
        println!("item={}", item);
    }

    println!("len={}", v.len());

    for i in 1..3 {
        println!("{}", i);
    }

    let p = PromtoType::PromtoB(22);
    print_promto(p);

    let s = "hello".to_owned();
    let ss = s;
    println!("{:?}", ss);
}
