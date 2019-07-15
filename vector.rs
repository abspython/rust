fn main() {
	let mut a:Vec<i32> = Vec::new();
	let mut b:Vec<i32>= vec![];

	let mut c: Vec<i32> = Vec::new();

	//Push abd popable
	c.push(1);
	c.push(2);
	c.pop();

	//Capacity and reallocation
	let mut e: Vec<i32> = Vec::with_capacity(10);
	println!("Length: {}, Capacity: {}", e.len(), e.capacity());

	for i in 0..10 {
		e.push(i);
	}
	e.push(11);
	println!("Length: {}, Capacity: {}", e.len(), e.capacity());

	let mut v = vec![1,2,3,4,5];

	for i in &v {
		println!("A reference to {}", i);
	}
}