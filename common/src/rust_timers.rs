static mut START: [f64; 64] = [0.0; 64];
static mut ELAPSED: [f64; 64] = [0.0; 64];
//use mylib::pega_tempo;
use super::pega_tempo;
pub fn timer_clear(x:usize){
    let elapsed: &'static mut [f64; 64] = unsafe { &mut ELAPSED };
    elapsed[x] = 0.0;
}
pub fn timer_start(x:usize){
    let start: &'static mut [f64; 64] = unsafe { &mut START };
    start[x] = pega_tempo::wtime();
}
pub fn timer_stop(x:usize){
    let elapsed: &'static mut [f64; 64] = unsafe { &mut ELAPSED };
    let start: &'static mut [f64; 64] = unsafe { &mut START };
    let now:f64 = pega_tempo::wtime();
    let elapse = now - start[x];
    elapsed[x] += elapse;
}

pub fn timer_read(x:usize) -> f64{
    let elapsed: &'static mut [f64; 64] = unsafe { &mut ELAPSED };
    elapsed[x]
}
