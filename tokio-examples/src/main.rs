pub mod blocking_call;
pub mod func;
pub mod mutex;
pub mod semaphore;

fn main() {
    // blocking_call::exec();
    // mutex::exec();
    semaphore::exec();
}
