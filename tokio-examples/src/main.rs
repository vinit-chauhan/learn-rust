pub mod barrier;
pub mod blocking_call;
pub mod func;
pub mod mutex;
pub mod notify;
pub mod semaphore;

fn main() {
    // blocking_call::exec();
    // mutex::exec();
    // semaphore::exec();
    // notify::exec();
    barrier::exec();
}
