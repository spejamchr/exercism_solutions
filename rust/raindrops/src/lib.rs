pub fn raindrops(n: u32) -> String {
    return num_str(n) + sound(n, 3, "Pling") + sound(n, 5, "Plang") + sound(n, 7, "Plong");
}

fn num_str(n: u32) -> String {
    if n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
        return "".to_owned();
    } else {
        return n.to_string();
    }
}

fn sound(n: u32, f: u32, sound: &str) -> &str {
    if n % f == 0 {
        return sound;
    } else {
        return "";
    }
}
