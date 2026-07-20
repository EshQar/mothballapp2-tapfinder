use std::cmp::Reverse;

pub fn sorted_by_key<I, F, K>(
    iter: I,
    key: F,
    reverse: bool,
) -> Vec<I::Item>
where
    I: IntoIterator,
    F: Fn(&I::Item) -> K,
    K: Ord,
{
    let mut v: Vec<_> = iter.into_iter().collect();

    if reverse {
        v.sort_by_key(|x| Reverse(key(x)));
    } else {
        v.sort_by_key(key);
    }

    v
}