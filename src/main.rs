#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod pcg;
use pcg::Rng;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const VGA_BUFFER: *mut u8 = 0xB8000 as *mut u8;

const HEIGHT: usize = 25;
const WIDTH: usize = 80;
const N_CELLS: usize = WIDTH * HEIGHT;

type GolBuffer = [bool; N_CELLS];

struct Conway {
    buf_a: GolBuffer,
    buf_b: GolBuffer,
}

fn random_gol(rng: &mut Rng, buf: &mut GolBuffer) {
    let n_bits = 32;
    for chunk in buf.chunks_mut(n_bits) {
        let bits = rng.gen();
        for (shift, bit) in chunk.iter_mut().enumerate() {
            *bit = (bits >> shift) & 1 == 0;
        }
    }
}

fn display_gol(gol: &GolBuffer) {
    for (i, &cell) in gol.iter().enumerate() {
        let color: u8 = match cell {
            true => 0x0b,
            false => 0x00,
        };
        let idx = 2 * i as isize;
        unsafe {
            *VGA_BUFFER.offset(idx) = b'#';
            *VGA_BUFFER.offset(idx+1) = color;
        }
    }
}

 fn hello() {
     let color: u8 = 0xb;
     static HELLO: &[u8] = b"HELLO";
     for (i, &b) in HELLO.iter().enumerate() {
         let idx = 2 * i as isize;
         unsafe {
             *VGA_BUFFER.offset(idx) = b;
             *VGA_BUFFER.offset(idx+1) = color;
         }
     }
 }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut color: u8 = 0;
    let mut rng = Rng::new();

    let mut buf_a = [false; N_CELLS];
    let mut buf_b = [false; N_CELLS];

    loop {
        random_gol(&mut rng, &mut buf_a);
        display_gol(&buf_a);
        for i in 0..2_000_000 {}
    }
}
