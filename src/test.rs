use std::cmp::Ordering;
use std::ops::Deref;
pub struct  BT<T>
where T: Ord,
{
    value: Option<T>,
    left: Option<Box<BT<T>>>,
    right: Option<Box<BT<T>>>,
}
impl<T> Default for BT<T>
where T:Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl <T> BT<T> where T:Ord
{
    pub fn new() -> BT<T>
    {
        BT{
            value:None,
            left:None,
            right:None,
        }
    }
}