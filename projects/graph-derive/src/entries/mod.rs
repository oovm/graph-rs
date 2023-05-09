use syn::Error;

pub enum ParseResult<T> {
    Ok(T),
    NotGood,
    Bad(Error),
}
