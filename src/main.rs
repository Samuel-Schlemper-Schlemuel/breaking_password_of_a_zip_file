use std::fs::File;
use std::io::{Read, BufReader};
use zip::read::ZipArchive;
use indicatif::ProgressBar;
use clap::{Parser, Subcommand};

/// A tool to break a password of a zip file
#[derive(Parser)]
pub struct Cli {
	/// Path to the protect zipped archive
	#[arg(short, long)]
	path_to_archive: Option<String>,

	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	/// Use the anagrams method
	Anagrams {
		/// Minimum number of letters in the password
		#[arg(short='i', long, default_value_t = 1)]
		min_letters_quantity: u32,

		/// Maximum number of letters in the password [default: equal to min_letters_quantity]
		#[arg(short='x', long)]
		max_letters_quantity: Option<u32>,

		/// Letters to try in the passwords ("abcABC")
		#[arg(short, long, default_value =
						   "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")]
		letters: String,
	},
	/// Use the dictionary method
	Dictionary {
		text: String,
	},
	/// Use the anagram dictionary method (more info in manual)
	AnagramDictionary {
		text: String
	},
	/// See the Manual
	Manual{},
}

fn anagrams(path: &str, letters: &str, zip_archive: &mut ZipArchive<File>,
                     password_size: usize, initial_string: &mut String,
                     progress_bar: &ProgressBar) -> bool {
    for i in 0..letters.chars().count() {
        let mut test_password = format!("{}{}", initial_string, letters.chars().nth(i)
                                                                .expect("Failed in get character")
                                                                .to_string());
        if test_password.chars().count() < password_size {
            if anagrams(path, letters, zip_archive, password_size,
                                 &mut test_password, progress_bar){
                return true
            }
        } else {
            progress_bar.inc(1);

            match zip_archive.by_index_decrypt(0, test_password.as_bytes()) {
                Err(..) => {},
                Ok(mut zip) => {
                    let mut buffer = Vec::with_capacity(zip.size() as usize);

                    match zip.read_to_end(&mut buffer) {
                        Err(..) => {},
                        Ok(..) => {
                            println!("The password is: {}", test_password);
                            return true
                        }
                    }
                }
            }
        }
    }
    return false
}

/*fn test_zip_archive(zip_path: &String) -> String {
	*zip_path = *zip_path.expect("Please, write a path to the zip encripted file");

	let mut zip_archive = match File::open(zip_path) {
		Ok(file) => ZipArchive::new(file).expect("Failed to open zip archive"),
		Err(e) => {
			eprintln!("Error opening zip file: {}", e);
			"Error";
		}
	};

	return zip_archive
}*/

fn main() {
	let args = Cli::parse();

	match &args.command {
		Some(Commands::Anagrams {min_letters_quantity, max_letters_quantity, letters}) => {
			let path_to_archive = args.path_to_archive.expect("Please, write a path to the zip \
															  encripted file");

			let mut zip_archive = match File::open(&path_to_archive) {
				Ok(file) => ZipArchive::new(file).expect("Failed to open zip archive"),
				Err(e) => {
					eprintln!("Error opening zip file: {}", e);
					return;
				}
			};

			let max_letters_quantity: &u32 = &max_letters_quantity
											.unwrap_or(*min_letters_quantity);

			if *min_letters_quantity < 1 {
				println!("min_letters_quantity must to be at least 1");
				return
			} else if min_letters_quantity > max_letters_quantity {
				println!("max_letters_quantity must to be greater than min_letters_quantity");
				return
			}

			let letters_quantity_on_user_string = letters.chars().count();

			let mut quantity_of_combinations = 0;
			for q in *min_letters_quantity..(*max_letters_quantity + 1) {
				quantity_of_combinations += letters_quantity_on_user_string.pow(q);
			}

			println!("Quantity of combintions: {}", quantity_of_combinations);

			for password_size in *min_letters_quantity..
						         (*max_letters_quantity + 1) {

				println!("Testing with {} letter(s)", password_size);
				let progress_bar = ProgressBar
						          ::new(letters_quantity_on_user_string.pow(password_size) as u64);

				let mut initial_string = String::new();
				if anagrams(&path_to_archive, &letters, &mut zip_archive,
						             password_size as usize, &mut initial_string, &progress_bar){
					return
				}
			}

			println!("The password wasn't found.");
			return
		}
		Some(Commands::Dictionary {text}) => {
			println!("{}", text);
		}
		Some(Commands::AnagramDictionary {text}) => {
			println!("{}", text)
		}
		Some(Commands::Manual {}) => {
			let file = File::open("src/Manuals/man.txt")
							.expect("Error in opening the manual file");
			let mut buf_reader = BufReader::new(file);
			let mut contents = String::new();
			let _ = buf_reader.read_to_string(&mut contents);
			println!("{}", contents);
		}
		&_ => println!("Found no option. Use --help if needed."),
	}
}
