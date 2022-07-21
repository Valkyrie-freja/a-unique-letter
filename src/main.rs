use std::io;

fn main() {
    //println!("input string > ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap()/*.expect("Failed to read line")*/;
    //println!("You input: {}", input);
    let chars: Vec<char> = input.chars().collect();
    let mut has_dbl:[bool; 3] = [false; 3];
    for i in 0..3 {
        for j in i..3-i {
            if i == j { continue; }
            if chars[i] == chars[j] {
                has_dbl[i] = true;
                has_dbl[j] = true;
            }
        }
    }

    for i in 0..3{
        if has_dbl[i] == false {
            println!("{}", chars[i]);
            return;
        }
    }
    println!("-1");
}