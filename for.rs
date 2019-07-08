fn main() {
	for a in 0..10{
		println!("Current Value : {}", a);
	}
	let group:[&str;4] = ["Mark","Jack","Pewds","Elon"];

/* Don't use the below code which is inefficient
	for n in 0..group.len(){
		println!("Current person : {}", group[n]);
	}
*/
	for person in group.iter(){
		println!("Current person : {}", person);
	}
}