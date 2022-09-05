use std::io;

fn main() {
    println!("Please, input F temperature");
    
    loop {
    	let mut input = String::new();

    	io::stdin().
		read_line(&mut input).
		expect("Err!");

    	let input: f64 = match input.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			println!("Unable to parse number! Please input correct num");
			continue;
		}
	};
	
    	println!("Celcius temperature is: {}", (input - 32.0) * 5.0/9.0);
	break;
    }
}
