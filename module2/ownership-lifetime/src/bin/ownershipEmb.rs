use std::cell::Cell;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

/* =====================================================
   Simulated Hardware Registers (NO external crates)
   ===================================================== */

// These behave like memory-mapped registers
static LED_CONTROL: AtomicU32 = AtomicU32::new(0);
static BUTTON_STATUS: AtomicU32 = AtomicU32::new(0);
static TIMER_COUNT: AtomicU32 = AtomicU32::new(0);
static INTERRUPT_FLAG: AtomicU32 = AtomicU32::new(0);

// Mutex is fine as a static
static PWM_DUTY: Mutex<u32> = Mutex::new(0);

/* =====================================================
   Hardware access functions
   ===================================================== */

fn write_led(val: u32) {
    LED_CONTROL.store(val, Ordering::SeqCst);
    println!("[HW] LED = {}", val);
}

fn read_button() -> u32 {
    BUTTON_STATUS.load(Ordering::SeqCst)
}

fn write_button(val: u32) {
    BUTTON_STATUS.store(val, Ordering::SeqCst);
}

fn write_pwm(val: u32) {
    let mut pwm = PWM_DUTY.lock().unwrap();
    *pwm = val;
    println!("[HW] PWM duty {}%", (val * 100) / 255);
}

fn increment_timer() {
    TIMER_COUNT.fetch_add(1, Ordering::SeqCst);
}

fn read_timer() -> u32 {
    TIMER_COUNT.load(Ordering::SeqCst)
}

fn set_interrupt(bit: u8) {
    INTERRUPT_FLAG.fetch_or(1 << bit, Ordering::SeqCst);
}

fn clear_interrupt(bit: u8) {
    INTERRUPT_FLAG.fetch_and(!(1 << bit), Ordering::SeqCst);
}

fn read_interrupts() -> u32 {
    INTERRUPT_FLAG.load(Ordering::SeqCst)
}

/* =====================================================
   LED Controller (Ownership enforced)
   ===================================================== */

struct LedController {
    _not_send_sync: PhantomData<Cell<()>>,
}

impl LedController {
    fn new() -> Self {
        write_led(0);
        Self {
            _not_send_sync: PhantomData,
        }
    }

    fn on(&mut self) {
        write_led(1);
    }

    fn off(&mut self) {
        write_led(0);
    }

    fn blink(&mut self, times: u32) {
        for _ in 0..times {
            self.on();
            self.delay(50);
            self.off();
            self.delay(50);
        }
    }

    fn delay(&self, ms: u32) {
        for _ in 0..ms {
            increment_timer();
        }
    }
}

impl Drop for LedController {
    fn drop(&mut self) {
        write_led(0);
        println!("LED released");
    }
}

/* =====================================================
   Button Reader (Volatile Read Demo)
   ===================================================== */

struct ButtonReader {
    last: bool,
}

impl ButtonReader {
    fn new() -> Self {
        Self { last: false }
    }

    fn is_pressed(&mut self) -> bool {
        let pressed = read_button() != 0;
        if pressed != self.last {
            println!("Button changed: {}", pressed);
            self.last = pressed;
        }
        pressed
    }
}

/* =====================================================
   Volatile vs Cached Read Demo
   ===================================================== */

struct CachedTimer {
    cache: Option<u32>,
}

impl CachedTimer {
    fn new() -> Self {
        Self { cache: None }
    }

    fn read_cached(&mut self) -> u32 {
        if let Some(v) = self.cache {
            println!("[BAD] cached = {}", v);
            v
        } else {
            let v = read_timer();
            self.cache = Some(v);
            v
        }
    }

    fn read_volatile(&self) -> u32 {
        let v = read_timer();
        println!("[GOOD] volatile = {}", v);
        v
    }
}

/* =====================================================
   MAIN
   ===================================================== */

fn main() {
    println!("=== SIMPLE EMBEDDED SIMULATION ===");

    // LED ownership
    {
        let mut led = LedController::new();
        led.blink(2);
    }

    // Button demo
    let mut button = ButtonReader::new();
    write_button(0);
    button.is_pressed();
    write_button(1);
    button.is_pressed();

    // PWM demo
    write_pwm(128);

    // Timer demo
    let mut timer = CachedTimer::new();
    increment_timer();
    increment_timer();

    timer.read_cached();
    increment_timer();
    timer.read_volatile();

    // Interrupt demo
    set_interrupt(2);
    println!("Interrupt flags: {}", read_interrupts());
    clear_interrupt(2);
}
