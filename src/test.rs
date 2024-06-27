use std::cmp::Ordering;
use std::ops::Deref;
pub struct  BT<T>
where T: Ord,
{
    value: Option<T>,
    left: Option<Box<BT<T>>>,
    right: Option<Box<BT<T>>>,
}