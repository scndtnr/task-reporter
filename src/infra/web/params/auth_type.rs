#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) enum AuthType {
    General,
    #[allow(unused)]
    Bearer,
}
