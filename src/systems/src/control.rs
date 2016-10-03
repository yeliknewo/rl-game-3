use dependencies::specs::{System, RunArg};
use event::{BackChannel};
use utils::{Delta};

#[derive(Debug)]
pub enum ToControl {

}

#[derive(Debug)]
pub enum FromControl {

}

#[derive(Debug)]
pub struct ControlSystem {
    back_channel: BackChannel<ToControl, FromControl>,
}

impl ControlSystem {
    pub fn new(back_channel: BackChannel<ToControl, FromControl>) -> ControlSystem {
        ControlSystem {
            back_channel: back_channel,
        }
    }
}

impl System<Delta> for ControlSystem {
    fn run(&mut self, arg: RunArg, _: Delta) {
        arg.fetch(|_| ());
    }
}
