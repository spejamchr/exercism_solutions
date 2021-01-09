pub fn verse(n: u32) -> String {
    return format!(
        "{}, {}.\n{}.\n",
        n_bottles_on_the_wall(n),
        n_bottles(n).to_lowercase(),
        second_verse(n)
    );
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .map(verse)
        .rev()
        .collect::<Vec<String>>()
        .join("\n")
}

fn n_bottles(n: u32) -> String {
    match n {
        0 => "No more bottles of beer".to_owned(),
        1 => "1 bottle of beer".to_owned(),
        n => format!("{} bottles of beer", n),
    }
}

fn n_bottles_on_the_wall(n: u32) -> String {
    format!("{} on the wall", n_bottles(n))
}

fn it_or_one(n: u32) -> String {
    match n {
        1 => "it".to_owned(),
        _ => "one".to_owned(),
    }
}

fn second_verse(n: u32) -> String {
    match n {
        0 => format!(
            "Go to the store and buy some more, {}",
            n_bottles_on_the_wall(99)
        ),
        n => format!(
            "Take {} down and pass it around, {}",
            it_or_one(n),
            n_bottles_on_the_wall(n - 1).to_lowercase()
        ),
    }
}
