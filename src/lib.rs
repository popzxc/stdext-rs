pub mod duration;
pub mod option;
pub mod result;
pub mod sync;
pub mod vec;

pub mod prelude {
    pub use crate::duration::*;
    pub use crate::option::*;
    pub use crate::result::*;
    pub use crate::sync::rw_lock::*;
    pub use crate::vec::*;
}
