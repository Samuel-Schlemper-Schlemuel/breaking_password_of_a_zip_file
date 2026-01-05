use std::env;
use std::fs::File;
use std::io::Read;
use zip::read::ZipArchive;
use indicatif::ProgressBar;

fn brute_force_break(path: &str, letters: &str, zip_archive: &mut ZipArchive<File>,
                     password_size: usize, initial_string: &mut String, 
                     progress_bar: &ProgressBar) -> bool {
    for i in 0..letters.chars().count() {
        let mut test_password = format!("{}{}", initial_string, letters.chars().nth(i)
                                                                .expect("Failed in get character")
                                                                .to_string());
        if test_password.chars().count() < password_size {
            if brute_force_break(path, letters, zip_archive, password_size, 
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

fn main() {
    println!("Use: ./bin min_letters_quantity max_letters_quantity letters path_to_archive");
    println!("Use: ./bin --help for more details");
    println!("Example: ./bin 1 8 \"abcdew\" \"/home/name/my archive.zip\"");
    println!("\n======================================================================\n");

    let args: Vec<String> = env::args().collect();

    if &args[1] == "--help" {
        println!("
It's necessary use 4 objects when calling this function (unless you are calling with the flag --help).
The first is min_leters_quantity, which means it's the min quantity of letters in the password.
The second is max_letters_quantity, which means it's the max quatity of letters in the password.
The third is letters, which means it's the letters used to build the password.
The fourth is path_to_archive, which means it's the path to the encripted zip archive.

Note that the min_letters_quantity must to be at least 1 and the max_letters_quantity must to be 
equal to or greater than min_letters_quantity.");
        return
    } else {
        let min_letters_quantity_on_password = args[1].parse()
                                                      .expect("Must to be a natural number");
        let max_letters_quantity_on_password = args[2].parse()
                                                      .expect("Must to be a natural number");

        if min_letters_quantity_on_password < 1 {
            println!("min_letters_quantity must to be at least 1");
            return
        } else if min_letters_quantity_on_password > max_letters_quantity_on_password {
            println!("max_letters_quantity must to be greater than min_letters_quantity");
            return
        }

        let letters = &args[3];
        let path_to_archive = &args[4];

        let letters_quantity_on_user_string = letters.chars().count();

        let mut quantity_of_combinations = 0;
        for q in min_letters_quantity_on_password..(max_letters_quantity_on_password as u32 + 1) {
            quantity_of_combinations += letters_quantity_on_user_string.pow(q);
        }

        println!("Quantity of combintions: {}", quantity_of_combinations);

        let mut zip_archive = match File::open(path_to_archive) {
            Ok(file) => ZipArchive::new(file).expect("Failed to open zip archive"),
            Err(e) => {
                eprintln!("Error opening zip file: {}", e);
                return;
            }
        };

        for password_size in min_letters_quantity_on_password..
                             (max_letters_quantity_on_password + 1) {
                        
            println!("Testing with {} letter(s)", password_size);
            let progress_bar = ProgressBar
                               ::new(letters_quantity_on_user_string.pow(password_size) as u64);

            let mut initial_string = String::new();
            if brute_force_break(&path_to_archive, &letters, &mut zip_archive, 
                                 password_size as usize, &mut initial_string, &progress_bar){
                return
            }
        }

        println!("The password wasn't found.");
        return
    }
}
