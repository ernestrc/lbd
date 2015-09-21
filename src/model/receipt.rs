use std::io;
use std::any::Any;

pub enum Receipt{
    Success{ id: i32, start_ts: i64, end_ts: i64 },
    Failure { msg: String, underlying: io::Result<()> }
}

