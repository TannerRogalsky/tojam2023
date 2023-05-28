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

type InputState = [bool; 8];
type InputStates = scapegoat::SgMap<u32, InputState, 25>;
static INPUT_STATES: spin::Lazy<spin::Mutex<InputStates>> =
    spin::Lazy::new(|| spin::Mutex::new(InputStates::new()));

extern "C" {
    #[allow(unused)]
    fn log_u32(a: u32);
    fn set_fb_ptr(ptr: *const u8);
    fn set_input_state(
        peer_id: u32,
        up: bool,
        down: bool,
        left: bool,
        right: bool,
        a: bool,
        b: bool,
        start: bool,
        select: bool,
    );
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
        let input_states = INPUT_STATES.lock();
        for (peer_id, state) in input_states.iter() {
            let [up, down, left, right, a, b, start, select] = *state;
            set_input_state(*peer_id, up, down, left, right, a, b, start, select);
        }
    }
}

#[no_mangle]
pub extern "C" fn set_button(peer_id: u32, button: u32, is_pressed: bool) {
    let padme_button = match button {
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

    let mut input_states = INPUT_STATES.lock();
    input_states.entry(peer_id).or_default()[button as usize] = is_pressed;

    let mut state = STATE.lock();
    state.set_button(padme_button, is_pressed);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
