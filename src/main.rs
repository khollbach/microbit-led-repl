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
    pac::{
        P0,
        P1
    },
    Peripherals, 
    Board
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use core::fmt::Write;
use core::str;
use void::ResultVoidExt;

const ROWS: [(usize, usize); 5] = [(0, 21), (0, 22), (0, 15), (0, 24), (0, 19)];
const COLS: [(usize, usize); 5] = [(0, 28), (0, 11), (0, 31), (1, 05), (0, 30)];

#[entry]
unsafe fn main() -> ! {
    static mut TX_BUF: [u8; 1] = [0u8];
    static mut RX_BUF: [u8; 1] = [0u8];

    rtt_init_print!();
    rprintln!("hi");

    //let p = Peripherals::take().unwrap();
    let mut board = Board::take().unwrap();

    //// Set pins to output mode.
    //let ports = [&*P0::ptr(), &*P1::ptr()];
    //for (p, i) in ROWS.into_iter().chain(COLS) {
        //ports[p].pin_cnf[i].write(|w| w.dir().output());
    //}

    //// Rows start low.
    //for (p, i) in ROWS {
        //ports[p].outclr.write(|w| w.bits(1 << i));
    //}

    //// Cols start high.
    //for (p, i) in COLS {
        //ports[p].outset.write(|w| w.bits(1 << i));
    //}

    // Setup serial port.
    //let p0 = p0::Parts::new(p.P0);
    //let p1 = p1::Parts::new(p.P1);
    let (mut tx, mut rx) = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        //Pins {
            //txd: p0.p0_06.degrade().into_push_pull_output(Level::High),
            //rxd: p1.p1_08.degrade().into_floating_input(),
            //cts: None,
            //rts: None,
        //},
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

            if b == b'\r' {
                nb::block!(tx.write(b'\r')).unwrap();
                nb::block!(tx.write(b'\n')).unwrap();
                tx.bflush().unwrap();
                break;
            }

            nb::block!(tx.write(b)).unwrap();
            tx.bflush().unwrap();

            buf.push(b).expect("buffer overflow");
        }

        match str::from_utf8(&buf).unwrap() {
            "c1" => {
                write!(tx, "got c1\r\n").unwrap();
                tx.bflush().unwrap();
                //let (p, i) = COLS[0];
                //ports[p].outclr.write(|w| w.bits(1 << i));
                board.display_pins.col1.set_low().void_unwrap();
            },
            "c2" => (),
            "c3" => (),
            "c4" => (),
            "c5" => (),
            "r1" => {
                //let (p, i) = ROWS[0];
                //ports[p].outset.write(|w| w.bits(1 << i));
                write!(tx, "got r1\r\n").unwrap();
                tx.bflush().unwrap();
                board.display_pins.row1.set_high().void_unwrap();
            },
            "r2" => (),
            "r3" => (),
            "r4" => (),
            "r5" => (),
            s => {
                write!(tx, "invalid command: {s:?}\r\n").unwrap();
                tx.bflush().unwrap();
            }
        }
        buf.clear();
    }
}
