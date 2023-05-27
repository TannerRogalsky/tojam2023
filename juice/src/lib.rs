#![no_std]
mod web;

type System = padme_core::System<&'static [u8], web::Lcd, web::SerialConsole, web::AudioPlayer>;

const ROM: &'static [u8] = include_bytes!("dangan.gb");
static STATE: spin::Lazy<spin::Mutex<System>> = spin::Lazy::new(|| {
    let rom = padme_core::Rom::load(ROM)
        .map_err(|_| "Could not parse ROM.")
        .unwrap();
    spin::Mutex::new(System::new(
        rom,
        web::Lcd::new(),
        web::SerialConsole::new(),
        web::AudioPlayer::new(),
    ))
});

extern "C" {
    #[allow(unused)]
    fn log_u32(a: u32);
    fn set_fb_ptr(ptr: *const u8);
}

#[no_mangle]
pub extern "C" fn fixed_update() {
    let mut state = STATE.lock();
    let _result = state.update_frame();
}

#[no_mangle]
pub extern "C" fn framebuffer() {
    let mut state = STATE.lock();
    let ptr = state.screen().framebuffer();
    unsafe {
        set_fb_ptr(ptr);
    }
}

#[no_mangle]
pub extern "C" fn set_button(button: u32, is_pressed: bool) {
    unsafe {
        log_u32(button);
    }
    let button = match button {
        0 => padme_core::Button::Up,
        1 => padme_core::Button::Down,
        2 => padme_core::Button::Left,
        3 => padme_core::Button::Right,
        4 => padme_core::Button::A,      // KeyQ
        5 => padme_core::Button::B,      // KeyW
        6 => padme_core::Button::Start,  // KeyA,
        7 => padme_core::Button::Select, // KeyS
        _ => return,
    };

    let mut state = STATE.lock();
    state.set_button(button, is_pressed);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
