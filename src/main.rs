use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_text: String = args[1..].join(" ").chars().rev().collect();
    let mut row1: Vec<String> = vec![];
    let mut row2: Vec<String> = vec![];
    let mut row3: Vec<String> = vec![];
    let mut row: u8 = 1;
    let mut direction: u8 = 0; // 0 = down, 1 = up
    for c in raw_text.chars() {
        if row == 1 {
            row1.push(c.to_string());
            row = 2;
        } else if row == 2 {
            row2.push(c.to_string());
            if direction == 0 {
                row = 3;
                direction = 1;
            } else {
                row = 1;
                direction = 0;
            }
        } else if row == 3 {
            row3.push(c.to_string());
            row = 2;
        }
   }
   let line1: String = format!("👻 {}", row1.join("         "));
   let line2: String = format!("     {}", row2.join("    "));
   let line3: String = format!("       {}", row3.join("         "));

   println!("{}", line1);
   println!("{}", line2);
   println!("{}", line3);
}
