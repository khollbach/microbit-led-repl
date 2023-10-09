#![no_std]
#![no_main]

use core::fmt::Write;
use core::str;
use cortex_m_rt::entry;
use microbit::{
    hal::{
        prelude::*,
        uarte::{Baudrate, Parity},
        Uarte,
    },
    Board,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use void::{ResultVoidExt, Void};

#[entry]
unsafe fn main() -> ! {
    static mut TX_BUF: [u8; 1] = [0u8];
    static mut RX_BUF: [u8; 1] = [0u8];

    rtt_init_print!();
    rprintln!("hi");

    let mut board = Board::take().unwrap();

    // Setup serial port.
    let (mut tx, mut rx) = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    )
    .split(TX_BUF, RX_BUF)
    .unwrap();

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

        // TODO: why is backspace breaking our text input code ?

        let pin: &mut dyn StatefulOutputPin<Error=Void> = match str::from_utf8(&buf).unwrap() {
            "c1" => &mut board.display_pins.col1,
            "c2" => &mut board.display_pins.col2,
            "c3" => &mut board.display_pins.col3,
            "c4" => &mut board.display_pins.col4,
            "c5" => &mut board.display_pins.col5,
            "r1" => &mut board.display_pins.row1,
            "r2" => &mut board.display_pins.row2,
            "r3" => &mut board.display_pins.row3,
            "r4" => &mut board.display_pins.row4,
            "r5" => &mut board.display_pins.row5,
            s => {
                write!(tx, "invalid command: {s:?}\r\n").unwrap();
                tx.bflush().unwrap();
                continue;
            }
        };
        buf.clear();

        if pin.is_set_high().void_unwrap() {
            pin.set_low().void_unwrap();
        } else {
            pin.set_high().void_unwrap();
        }

        // TODO: why is toggle not available ?
        // pin.togg
    }
}
