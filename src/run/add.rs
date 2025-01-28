use crate::{db::DB, AddArgs};

use super::RunResult;

pub fn adder(args: AddArgs) -> RunResult {
    let mut db = DB::new()?;
    db.new_note(&args.title, &args.content)?;
    Ok(())
}
