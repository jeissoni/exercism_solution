pub fn verse(n: u32) -> String {
    match n {
        0 => format!(
            "No more bottles of beer on the wall, no more bottles of beer.\n\
            Go to the store and buy some more, 99 bottles of beer on the wall.\n"
        ),
        1 => format!(
            "1 bottle of beer on the wall, 1 bottle of beer.\n\
            Take it down and pass it around, no more bottles of beer on the wall.\n"
        ),
        _ => format!(
            "{} bottles of beer on the wall, {} bottles of beer.\n\
            Take one down and pass it around, {} bottle{} of beer on the wall.\n",
            n, n, n - 1, if n > 2 { "s" } else { "" }
        ),
    }
}


pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();

    for num in (end..=start).rev() {
        song.push_str(&verse(num));
        if num != end {
            song.push('\n');
        }
    }

    song
}