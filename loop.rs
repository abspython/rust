fn main() {
	let mut a  =0;
	loop{
		if a == 0{
			println!("Skip Value : {}", a);
			a += 1;
			continue;
		}else if a ==2 {
			println!("Break At : {}", a);
		    break;
		}
		println!("Current value : {}", a);
		a += 1;
	}
}