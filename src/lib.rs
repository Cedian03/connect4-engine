pub mod error;
pub mod position;
pub mod prelude;
pub mod solver;
pub mod util;

mod magic;

use magic::{MagicStruct, MagicTrait};
use position::Position;

pub type BitMask = <MagicStruct<{ Position::bits_required() }> as MagicTrait>::MagicType;
