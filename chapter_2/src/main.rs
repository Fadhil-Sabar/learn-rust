use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Tebak Nomor");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Masukkan nomor tebakan anda");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error saat membaca input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => return,
        };

        println!("Anda menebak {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Terlalu kecil!"),
            Ordering::Greater => println!("Terlalu besar!"),
            Ordering::Equal => {
                println!("Anda benar, angkanya adalah {secret_number}");
                break;
            }
        }
    }
}
