pub trait AsVec {
    type Item;
    fn into_inner(self) -> Vec<Self::Item>;
    fn as_vec(&self) -> &Vec<Self::Item>;
    fn as_mut_vec(&mut self) -> &mut Vec<Self::Item>;
    fn is_empty(&self) -> bool {
        self.as_vec().is_empty()
    }

    /// 引数のitemを自身から除外する
    fn exclude(&mut self, other: &Self::Item)
    where
        Self::Item: PartialEq,
    {
        self.as_mut_vec().retain(|item| item != other)
    }
}
