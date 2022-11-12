use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target};

use crate::data::AppState;

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        Handled::No
    }
}
