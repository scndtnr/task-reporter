#[derive(Debug, Clone)]
pub(crate) enum HttpMethods {
    Get,
    Post,
    #[allow(unused)]
    Put,
    #[allow(unused)]
    Patch,
    #[allow(unused)]
    Delete,
}
