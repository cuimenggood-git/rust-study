struct Person {
    age: Option<i32>,
    name: String,
}
fn main() {
    let stus = vec![
        Person {
            age: Some(12),
            name: "lili".to_owned(),
        },
        Person {
            age: None,
            name: "hanhan".to_owned(),
        },
    ];

    for su in stus {
        match su.age {
            Some(age) => {
                println!("name:{},age={}", age, su.name)
            }
            None => {
                println!("name:{} age field is none", su.name)
            }
        }
    }
}
