pub mod duration;
pub mod option;
pub mod result;
pub mod str;
pub mod sync;
pub mod vec;

pub mod prelude {
    pub use crate::{
        duration::*,
        option::*,
        result::*,
        str::*,
        sync::{mutex::*, rw_lock::*},
        vec::*,
    };
}
