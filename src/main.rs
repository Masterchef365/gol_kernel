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
            true => 0x0c,
            false => 0x00,
        };
        let idx = 2 * i as isize;
        unsafe {
            *VGA_BUFFER.offset(idx) = b'#';
            *VGA_BUFFER.offset(idx+1) = color;
        }
    }
}

fn in_bounds(x: isize, y: isize) -> bool {
    x >= 0 && y >= 0 && x < WIDTH as isize && y < HEIGHT as isize
}

fn gol_index(x: isize, y: isize) -> Option<usize> {
    in_bounds(x, y).then(|| x as usize + y as usize * WIDTH)
}

fn read_gol(buf: &GolBuffer, x: isize, y: isize) -> Option<bool> {
    gol_index(x, y).map(|idx| buf[idx])
}

fn write_gol(buf: &mut GolBuffer, x: isize, y: isize, value: bool) {
    if let Some(idx) = gol_index(x, y) {
        buf[idx] = value;
    }
}

fn read_neighbors(buf: &GolBuffer, x: isize, y: isize) -> u8 {
    let mut n = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            if let Some(true) = read_gol(buf, x + dx, y + dy) {
                n += 1;
            }
        }
    }
    n
}

fn gol_rules(neighbors: u8, cell: bool) -> bool {
    match (neighbors, cell) {
        (3, false) => true,
        (2 | 3, c) => c,
        _ => false,
    }
}

fn step_gol(front: &mut GolBuffer, back: &GolBuffer) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let (x, y) = (x as isize, y as isize);
            let neighbors = read_neighbors(back, x, y);
            let cell = read_gol(back, x, y).unwrap();
            let rule = gol_rules(neighbors, cell);
            write_gol(front, x, y, rule);
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut rng = Rng::new();

    let mut buf_a = [false; N_CELLS];
    let mut buf_b = [false; N_CELLS];

    let sleep = || for _ in 0..2_000_000 {};
    //let sleep = || ();

    loop {
        random_gol(&mut rng, &mut buf_a);
        for _ in 0..90*8 {
            step_gol(&mut buf_b, &buf_a);
            display_gol(&buf_b);
            sleep();
            step_gol(&mut buf_a, &buf_b);
            display_gol(&buf_a);
            sleep();
        }
    }
}
