pub mod duration;
pub mod option;
pub mod result;
pub mod sync;
pub mod vec;

pub mod prelude {
    pub use crate::{
        duration::*,
        option::*,
        result::*,
        sync::{mutex::*, rw_lock::*},
        vec::*,
    };
}
