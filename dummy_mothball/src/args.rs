pub fn handle_keyword_arg<T>(keyword_arg : Option<T>, default: T) -> T {
    match keyword_arg {
        Some(thing) => thing,
        None => default,
    }
}

pub fn handle_keyword_args<T>(keyword_args : Vec<Option<T>>, defaults: Vec<T>) -> Vec<T> {
    keyword_args.into_iter().zip(defaults).map(|(arg, default)| handle_keyword_arg(arg, default)).collect()
}