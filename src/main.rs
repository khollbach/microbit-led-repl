#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    hal::{
        prelude::*,
        gpio::{p0, p1, Level},
        uarte::{Baudrate, Parity, Pins},
        Uarte,
    },
    Peripherals, 
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use core::fmt::Write;
use core::str;

const ROWS: [(usize, usize); 5] = [(0, 21), (0, 22), (0, 15), (0, 24), (0, 19)];
const COLS: [(usize, usize); 5] = [(0, 28), (0, 11), (0, 31), (1, 05), (0, 30)];

#[entry]
unsafe fn main() -> ! {
    static mut TX_BUF: [u8; 1] = [0u8];
    static mut RX_BUF: [u8; 1] = [0u8];

    rtt_init_print!();
    rprintln!("hi");

    let p = Peripherals::take().unwrap();

    // Set pins to output mode.
    let ports = [&*p.P0, &*p.P1];
    for (p, i) in ROWS.into_iter().chain(COLS) {
        ports[p].pin_cnf[i].write(|w| w.dir().output());
    }

    // Rows start low.
    for (p, i) in ROWS {
        ports[p].outclr.write(|w| w.bits(1 << i));
    }

    // Cols start high.
    for (p, i) in COLS {
        ports[p].outset.write(|w| w.bits(1 << i));
    }

    // Setup serial port.
    let p0 = p0::Parts::new(p.P0);
    let p1 = p1::Parts::new(p.P1);
    let (mut tx, mut rx) = Uarte::new(
        p.UARTE0,
        Pins {
            txd: p0.p0_06.degrade().into_push_pull_output(Level::High),
            rxd: p1.p1_08.degrade().into_floating_input(),
            cts: None,
            rts: None,
        },
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    ).split(TX_BUF, RX_BUF).unwrap();

    // todo: print a command prompt in a loop; echo keystrokes
    // (can copy this from the tutorial code)

    // next: hook up the relevant commands to toggle LED states.

    let mut buf = heapless::Vec::<u8, 100>::new();

    loop {
        loop {
            let b = nb::block!(rx.read()).unwrap();
            if b == b'\r' || b == b'\n' {
                break;
            }
            // tx.write(b).unwrap();
            // tx.flush().unwrap();

            buf.push(b).expect("buffer overflow");
        }

        match str::from_utf8(&buf).unwrap() {
            "c1" => (),
            "c2" => (),
            "c3" => (),
            "c4" => (),
            "c5" => (),
            "r1" => (),
            "r2" => (),
            "r3" => (),
            "r4" => (),
            "r5" => (),
            s => {
                writeln!(tx, "invalid command: {s:?}").unwrap();
                tx.bflush().unwrap();
            }
        }
        buf.clear();
    }
}
