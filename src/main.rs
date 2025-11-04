//! Minimal serial interrupt read proof of concept
//!
//! This simple application:
//! 1. Sets up USB Serial JTAG with interrupts
//! 2. Reads characters in the interrupt handler
//! 3. Prints debug information about each byte received

#![no_std]
#![no_main]

use core::cell::RefCell;
use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    usb_serial_jtag::UsbSerialJtag,
};
use esp_hal_procmacros::handler;
use esp_println::println;
use riscv_rt::entry;

esp_bootloader_esp_idf::esp_app_desc!();

static USB_SERIAL: Mutex<RefCell<Option<UsbSerialJtag<'static, esp_hal::Blocking>>>> =
    Mutex::new(RefCell::new(None));

static BUFFER: Mutex<RefCell<[u8; 128]>> = Mutex::new(RefCell::new([0u8; 128]));
static BUFFER_POS: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_160MHz);
    let peripherals = esp_hal::init(config);

    println!("\n========================================");
    println!("Serial Interrupt Read Debug Tool");
    println!("========================================\n");

    let delay = Delay::new();

    let mut usb_serial = UsbSerialJtag::new(peripherals.USB_DEVICE);
    usb_serial.set_interrupt_handler(usb_device_handler);
    usb_serial.listen_rx_packet_recv_interrupt();

    critical_section::with(|cs| USB_SERIAL.borrow_ref_mut(cs).replace(usb_serial));

    println!("Type characters and press Enter to see them echoed back\n");

    loop {
        delay.delay_millis(1000);
    }
}

#[handler]
fn usb_device_handler() {
    critical_section::with(|cs| {
        let mut usb_serial = USB_SERIAL.borrow_ref_mut(cs);
        if let Some(usb_serial) = usb_serial.as_mut() {
            let mut buffer_ref = BUFFER.borrow_ref_mut(cs);
            let mut pos_ref = BUFFER_POS.borrow_ref_mut(cs);
            let pos = *pos_ref;

            while let nb::Result::Ok(byte) = usb_serial.read_byte() {
                println!("[DEBUG] Read byte: 0x{:02X} ({})", byte, byte as char);

                // Check for newline (Enter key)
                if byte == b'\n' || byte == b'\r' {
                    println!("\n[DEBUG] Got newline! Buffer contents ({} bytes):", pos);
                    println!("[DEBUG] Raw bytes: {:?}", &buffer_ref[..pos]);
                    if let Ok(s) = core::str::from_utf8(&buffer_ref[..pos]) {
                        println!("[DEBUG] As string: \"{}\"", s);
                    } else {
                        println!("[DEBUG] Not valid UTF-8");
                    }
                    println!();

                    // Reset buffer
                    *pos_ref = 0;
                    buffer_ref.fill(0);
                    continue;
                }

                // Handle backspace (both ASCII 127 DEL and 8 BS)
                if byte == 127 || byte == 8 {
                    if pos > 0 {
                        *pos_ref -= 1;
                        buffer_ref[*pos_ref] = 0;
                        println!("[DEBUG] Backspace, new position: {}", *pos_ref);
                    }
                    continue;
                }

                // Only accept printable ASCII characters (32-126)
                if (32..=126).contains(&byte) {
                    if pos < buffer_ref.len() {
                        buffer_ref[pos] = byte;
                        *pos_ref += 1;
                        println!("[DEBUG] Added to buffer, position now: {}", *pos_ref);
                    } else {
                        println!("[DEBUG] Buffer full!");
                    }
                }
            }

            usb_serial.reset_rx_packet_recv_interrupt();
        }
    });
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("\n\n========================================");
    println!("!!! PANIC !!!");
    println!("========================================");
    println!("{}", info);
    println!("========================================\n");

    loop {}
}
