//! wait what does this do

use std::fmt;

struct Structure(i32);
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let pizza = 2 ^ 100;
    println!("pizza says hi {}", pizza);
    let this = "nothing";
    println!("alright powers don't work. how about {:?}", this);

    let hello = Structure(123);
    println!("uhh {} okay what the hell is an i32 type", hello);
    //try!(println!("uhh {} okay what the hell is an i32 type", this));

    println!("ok alright screw that. let's see this {}", 1 - 2i32);

    let character: char = 'a';

    println!("can we do this? {} okay", character);

    let decimal = character as u8;
    println!("{} problems, and a ditch ain't one", decimal);

    println!("oh this is fun: {}", std::mem::size_of_val(&character));

    println!("pizza makes you fatter though: {}", std::mem::size_of_val(&(pizza as f64))); // ha ha you like that?
}
