pub mod import;
pub mod mutex;
pub mod queue;

use import::osDelay;


pub fn poll_for_result<F>(poll_step: usize, max_tick: usize, mut func: F) -> bool
where
    F: FnMut() -> bool,
{
    let mut tick = 0 as usize;
    loop {
        tick += poll_step;
        if tick >= max_tick {
            return false;
        }
        if func() {
            return true;
        }
        unsafe { osDelay(poll_step) };
    }
}
