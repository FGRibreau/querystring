pub type QueryParam<'a> = (&'a str, &'a str);
pub type QueryParams<'a> = Vec<QueryParam<'a>>;

/// Produces a URL query string from a given query by iterating through the vec.
///
/// # Examples
///
/// ```
/// extern crate querystring;
///
/// assert_eq!(querystring::stringify(vec![("foo", "bar"), ("baz", "qux")]), "foo=bar&baz=qux");
/// ```
pub fn stringify(query: QueryParams) -> String {
    query
        .iter()
        .fold(String::new(), |acc, &tuple| {
            acc + tuple.0 + "=" + tuple.1 + "&"
        })
        .trim_end_matches('&')
        .to_string()
}

/// Parses a given query string back into a vector of key-value pairs.
/// Extra/invalid strings will be ignored.
///
/// # Examples
///
/// ```
/// extern crate querystring;
///
/// assert_eq!(querystring::querify("foo=bar&baz=qux&"), vec![("foo", "bar"), ("baz", "qux")]);
/// assert_eq!(
///     querystring::querify("a=b&b=c&something_else=another-thing&notright#&blank=&ignoreme!"),
///     vec![
///         ("a", "b"),
///         ("b", "c"),
///         ("something_else", "another-thing"),
///         ("blank", ""),
///     ]);
/// assert_eq!(querystring::querify("arbitrary string"), vec![]);
/// ```
pub fn querify(string: &str) -> QueryParams {
    let mut v = Vec::new();
    for pair in string.split('&') {
        let mut it = pair.split('=').take(2);
        let kv = match (it.next(), it.next()) {
            (Some(k), Some(v)) => (k, v),
            _ => continue,
        };
        v.push(kv);
    }
    v
}
