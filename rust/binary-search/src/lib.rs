use std::cmp::Ordering;

pub fn find<T: Ord, A: AsRef<[T]>>(array: A, key: T) -> Option<usize> {
    let arr = array.as_ref();
    let mid = arr.len() / 2;

    match arr.get(mid).map(|n| key.cmp(n)) {
        None => None,
        Some(Ordering::Less) => find(&arr[..mid], key),
        Some(Ordering::Equal) => Some(mid),
        Some(Ordering::Greater) => find(&arr[mid + 1..], key).map(|i| i + mid + 1),
    }
}
