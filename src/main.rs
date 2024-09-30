use std::f32::consts;

fn main() {
    println!(
        "{}",
        rainbowise("What you guys are referring to as Linux, is in fact, GNU/Linux, or as I've recently taken to calling it, GNU plus Linux. Linux is not an operating system unto itself, but rather another free component of a fully functioning GNU system made useful by the GNU corelibs, shell utilities and vital system components comprising a full OS as defined by POSIX. Many computer users run a modified version of the GNU system every day, without realizing it. Through a peculiar turn of events, the version of GNU which is widely used today is often called \"Linux\", and many of its users are not aware that it is basically the GNU system, developed by the GNU Project. There really is a Linux, and these people are using it, but it is just a part of the system they use. Linux is the kernel: the program in the system that allocates the machine's resources to the other programs that you run. The kernel is an essential part of an operating system, but useless by itself; it can only function in the context of a complete operating system. Linux is normally used in combination with the GNU operating system: the whole system is basically GNU with Linux added, or GNU/Linux. All the so-called \"Linux\" distributions are really distributions of GNU/Linux.".
            to_string(), 0.1, 3.0)
        );
}

fn rainbowise(input: String, freq: f32, spread: f32) -> String {
    input
        .bytes()
        .into_iter()
        .enumerate()
        .map(|(i, chr)| {
            let (red, green, blue) = taste(freq, i as f32 / spread);
            format!("\x1b[38;2;{};{};{}m{}", red, green, blue, chr as char)
        })
        .collect::<Vec<String>>()
        .join("")
}

// inspired from https://github.com/busyloop/lolcat/blob/f4cca5601ea57df2b5b3c98feea8ad05f4421039/lib/lolcat/lol.rb#L36
fn taste(freq: f32, i: f32) -> (i32, i32, i32) {
    let red = ((freq * i).sin() * 127.0 + 128.0) as i32;
    let green = ((freq * i + 2.0 * consts::PI / 3.0).sin() * 127.0 + 128.0) as i32;
    let blue = ((freq * i + 4.0 * consts::PI / 3.0).sin() * 127.0 + 128.0) as i32;

    (red, green, blue)
}
