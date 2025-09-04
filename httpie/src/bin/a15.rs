trait Move {
    fn move_to(&self, x: f64, y: f64);
}

#[derive(Debug)]
struct Snake;
impl Move for Snake {
    fn move_to(&self, x: f64, y: f64) {
        println!("snake move to x:{},y:{}", x, y);
    }
}

impl Default for Snake {
    fn default() -> Self {
        Snake {}
    }
}

fn run_move(m: &impl Move, x: f64, y: f64) {
    m.move_to(x, y);
}

fn run_move_f2<T>(m: &T, x: f64, y: f64)
where
    T: Move + std::fmt::Debug,
{
    m.move_to(x, y);
}

fn main() {
    let s = Snake::default();
    run_move(&s, 1.23, 2.23);
    run_move_f2(&s, 1.23, 3.23);
}
