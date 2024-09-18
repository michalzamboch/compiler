#![allow(dead_code)]

use std::fmt::Debug;

pub trait StringInput: Debug + Send {
    fn get() -> String;
}
