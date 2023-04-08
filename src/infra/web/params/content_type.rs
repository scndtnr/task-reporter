#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) enum ContentType {
    Json,
    XWwwFormUrlencoded,
}
