use super::{Mount, NULL3args, Result};

impl Mount {
    pub fn null(&self) -> Result<()> {
        let args = NULL3args {};
        self._null(args)
    }
}
