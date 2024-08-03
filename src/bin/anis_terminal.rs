use ansi_term::{Colour,Style};
fn main() {
    //打印彩色文本到终端
    println!("This is {} in color,{} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green"));
    println!("{}, {} and {}",
             Colour::Yellow.paint("This is colored"),
             Style::new().bold().paint("this is bold"),
             Colour::Yellow.bold().paint("this is bold and colored"));
}