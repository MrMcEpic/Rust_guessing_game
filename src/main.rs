use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::thread;
use std::time;
use wincolor::{Console, Color, Intense};
use winconsole::console;


fn main(){
	text();
	total();
}

fn text(){
	let mut con = Console::stdout().unwrap();
	console::set_title("Guessing Game").unwrap();

	if let Some((w, _h)) = term_size::dimensions() {
		con.fg(Intense::Yes, Color::Cyan).unwrap();
		thread::sleep(time::Duration::from_millis(500));

		println!("{holder:^width$}", width=w, holder="~-==Guess the number==-~");

		con.reset().unwrap();
		thread::sleep(time::Duration::from_millis(1000));
		con.fg(Intense::Yes, Color::Green).unwrap();

		println!("{holder:^width$}", width=w, holder="~-==Now built in Rust!==-~");

		thread::sleep(time::Duration::from_millis(1000));
		con.reset().unwrap();
	} else {
		con.fg(Intense::Yes, Color::Cyan).unwrap();
		thread::sleep(time::Duration::from_millis(500));

		println!("{holder:^width$}", width=110, holder="~-==Guess the number==-~");

		con.reset().unwrap();
		thread::sleep(time::Duration::from_millis(1000));
		con.fg(Intense::Yes, Color::Green).unwrap();

		println!("{holder:^width$}", width=110, holder="~-==Now built in Rust!==-~");

		thread::sleep(time::Duration::from_millis(1000));
		con.reset().unwrap();
	}	
}

fn total() {
    let comp_num = rand::thread_rng().gen_range(1,101);
	let mut tracker = 0;
	
	loop{
		println!("Please input your guess. 1-100");

		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Not a number");
				continue;
			}
		};

		tracker = tracker + 1;

		if tracker == 1{
			println!("You guessed: {}. You've made 1 guess", guess);
		}else{
			println!("You guessed: {}. You've made {} guesses", guess, tracker);
		}

		let win = logic(tracker, comp_num, guess);
		if win == false{
			continue;
		}else{
			let mut play_again = String::new();

			println!("Play Again? Y/N");

			io::stdin().read_line(&mut play_again)
				.expect("Invalid input");
			
			play_again = play_again.to_ascii_uppercase();
			play_again = play_again.trim().to_string();

			if play_again == "Y" {
				break total();
			}else if play_again == "N" {
				println!("Closing in 2 seconds.");
				thread::sleep(time::Duration::from_secs(2));
				break;
			}
		}
	}
}

fn logic(tracker: u32, comp_num: u32, guess: u32) -> bool {
	match guess.cmp(&comp_num){
		Ordering::Less => {
			println!("Too Small");
			false
		},
		Ordering::Equal => {
			if tracker == 1{
				println!("You Win! With 1 guess!");
			}else{
				println!("You Win! With {} guesses", tracker);
			}
			thread::sleep(time::Duration::from_millis(500));
			true
		},
		Ordering::Greater => {
			println!("Too Big");
			false
		},
	}
}