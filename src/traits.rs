use std::fmt::Debug;

pub(crate) use id_derive::ID;

pub trait ID: Copy + Ord + From<i64> + Debug {
    fn id(&self) -> i64;
}
