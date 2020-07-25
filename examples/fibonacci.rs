fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 1 {
		panic!("Must provide number for fibonacci sequence");
	}
	let n = &args[0].parse::<i32>().unwrap();
	println!("{}", fibonacci(n));
}

fn fibonacci(n: i32) -> i32 {
	if n <= 0 {
		panic!("Must provide a positive number for fibonacci sequence");
	} else if n == 1 {
		0
	} else if n == 2 {
		1
	} else {
		fibonacci(n - 1) + fibonacci(n - 2)
	}
}