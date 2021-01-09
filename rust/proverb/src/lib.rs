pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2)
        .map(proverb_line)
        .chain(list.get(0).map(proverb_tail))
        .collect()
}

fn proverb_line(c: &[&str]) -> String {
    format!("For want of a {} the {} was lost.\n", c[0], c[1])
}

fn proverb_tail(item: &&str) -> String {
    format!("And all for the want of a {}.", item)
}
