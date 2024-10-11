use std::io;

fn main() {
    println!("hikariDB");

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("{}", input);
            }

            Err(n) =>  {
                println!("we not gucci {}", n)
            }
        }
    }
}
