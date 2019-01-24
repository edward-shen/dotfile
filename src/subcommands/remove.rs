use std::io::Error;

use crate::Context;

pub fn handler(context: Context) -> Result<(), Error> {
    dbg!(context);
    unimplemented!("Add command is complicated, so this will be done later")
}
