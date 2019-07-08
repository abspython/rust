fn main() {
	let mut a = 1;
	while a <= 10 {
		println!("Current value : {}", a);
		a += 1;
	}

    //Nested while
	let mut c = 1;
	'outer_while: while c < 6 {
		let mut b = 1;
		'inner_while:while b < 6{
			println!("Current value : [{}][{}]",c,b);
			if c == 2 && b == 2 { break 'outer_while; }
			b += 1;
		}
		c += 1;
	}
}