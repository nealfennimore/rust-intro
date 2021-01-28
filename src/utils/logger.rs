use std::fmt::Debug;
use std::any::Any;

pub fn debug<T: Any + Debug>(value: T) {
    println!("{:#?}", value)
}
