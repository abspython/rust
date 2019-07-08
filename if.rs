fn main(){
	let team_size = 7;
/**
Normal way of if-elseif-else
	if team_size<5{
		println!("Small");
	}else if team_size<10{
		println!("Medium");
	}else{
		println!("Large");
	}
**/
//Optimistic Code
	let team_size_text = if team_size < 5{
		"Small"
	}else if team_size < 10 {
		"Medium"
	}else {
		"Large"
	};
	println!("Current team size : {}",team_size_text);
	//One liner
	let isBelowEighteen = if team_size < 18 { true } else { false };
}