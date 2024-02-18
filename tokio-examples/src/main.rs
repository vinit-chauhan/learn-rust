pub mod barrier;
pub mod blocking_call;
pub mod func;
pub mod mp_sc_channel;
pub mod mutex;
pub mod notify;
pub mod one_shot_channel;
pub mod semaphore;

fn main() {
    // blocking_call::exec();
    // mutex::exec();
    // semaphore::exec();
    // notify::exec();
    // barrier::exec();

    // one_shot_channel::exec();
    mp_sc_channel::exec();
}
