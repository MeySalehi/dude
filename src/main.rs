use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let out = b"Hey Dude, I'm Ready to Go!.";
    let width = 28;

    let mut writter = BufWriter::new(stdout.lock());

    say(out, width, &mut writter).unwrap();
}
