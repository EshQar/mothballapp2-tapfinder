pub fn handle_keyword_arg<T>(keyword_arg : Option<T>, default: T) -> T {
    match keyword_arg {
        Some(thing) => thing,
        None => default,
    }
}