use rand::seq::IndexedRandom;

fn main() {
	println!("{:?}", "other bin started");
	let a = vec!['a', 'b', 'c','d'];

	let mut rg = rand::rng();
	for _ in 0..6{
		print!("{}", a.choose(&mut rg).unwrap());
	}
}