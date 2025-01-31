mod add;
mod get;
mod list;

pub use add::adder;
pub use get::getter;
pub use list::lister;

use crate::types::LaResult;

type RunResult = LaResult<()>;
