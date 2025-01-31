use crate::{
    types::{Note, DB},
    AddArgs,
};

use super::RunResult;

pub fn adder(args: AddArgs) -> RunResult {
    let db = DB::new_rc()?;
    Note::new(db, &args.title, &args.content, [].into())?;
    Ok(())
}
