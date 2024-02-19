pub mod barrier;
pub mod blocking_call;
pub mod channel;
pub mod func;
pub mod mutex;
pub mod notify;
pub mod semaphore;

pub use crate::channel::mp_sc_channel;
pub use crate::channel::one_shot_channel;
pub use crate::channel::watch_channel;

use crate::channel::watch_channel::exec as watch_channel_exec;

fn main() {
    // blocking_call::exec();
    // mutex::exec();
    // semaphore::exec();
    // notify::exec();
    // barrier::exec();

    // one_shot_channel::exec();
    // mp_sc_channel::exec();
    // watch_channel::exec();
    watch_channel_exec();
}
