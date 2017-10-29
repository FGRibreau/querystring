pub type QueryParam<'a> = (&'a str, &'a str);
pub type QueryParams<'a> = Vec<QueryParam<'a>>;

/// Produces a URL query string from a given query by iterating through the vec.
///
/// # Examples
///
/// ```
/// extern crate querystring;
///
/// assert_eq!(querystring::stringify(vec![("foo", "bar"), ("baz", "qux")]), "foo=bar&baz=qux&");
/// ```
pub fn stringify(query: QueryParams) -> String {
    query.iter().fold(String::new(), |acc, &tuple| {
        acc + tuple.0 + "=" + tuple.1 + "&"
    })
}
