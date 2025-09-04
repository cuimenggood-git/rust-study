use rand::seq::IndexedRandom;
use rand::Rng;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn random<R: Rng>(rng: &mut R) -> Self {
        Point {
            x: rng.random(),
            y: rng.random(),
        }
    }
}

fn main() {
    println!("this is cmd 1");
    let mut rng = rand::rng();
    let rand_tuple = rng.random::<(i32, bool, f64)>();
    let rand_point = Point::random(&mut rng);
    println!("{:?}", rand_tuple);
    println!("x={},y={}", rand_point.x, rand_point.y);
    let passwd = gen_str();
    println!("{:?}", passwd);
    let passwd = gen_str();
    println!("{:?}", passwd);
    let a = vec!["a", "b", "c"];
    let b = &a[0..2];
    let c = &a[2..];
    println!("a={:?},b={:?}", b, c);

    let _s1 = String::from("abcd");

    // let result;
    // {
    //         let s2<'a> = String::from("xyz");
    //     result = logest(s1.as_str(), s2.as_str());
    // }

    // println!("{:?}", result);
    get_random();
}

fn gen_str() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::rng();
    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    password
}

fn logest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn get_random() {
    let mut rg = rand::rng();
    let chars = vec!['a', 'c', 'e', 'r', 't', 'y', 'q'];
    let res = chars.choose(&mut rg).unwrap();
    println!("res={:?}", res);
}
