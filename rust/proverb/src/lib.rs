pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return "".to_owned();
    }

    let mut proverb = proverb_body(list);
    proverb.push(proverb_tail(list));
    proverb.join("\n")
}

fn proverb_tail(list: &[&str]) -> String {
    format!("And all for the want of a {}.", list[0])
}

fn proverb_body(list: &[&str]) -> Vec<String> {
    list.iter()
        .zip(list[1..].iter())
        .map(proverb_line)
        .collect()
}

fn proverb_line((a, b): (&&str, &&str)) -> String {
    format!("For want of a {} the {} was lost.", a, b)
}
