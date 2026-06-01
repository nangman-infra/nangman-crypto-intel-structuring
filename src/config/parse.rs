use super::Args;
use crate::error::{AppError, AppResult};

mod option;
mod value;

impl Args {
    pub fn parse<I>(mut values: I) -> AppResult<Self>
    where
        I: Iterator<Item = String>,
    {
        let _program = values.next();
        let mut args = Self::from_env()?;

        while let Some(arg) = values.next() {
            match option::apply_cli_option(&mut args, &mut values, &arg)? {
                option::ParseAction::Continue => {}
                option::ParseAction::Help => return Err(AppError::config(value::help())),
            }
        }

        args.validate()?;
        Ok(args)
    }
}
