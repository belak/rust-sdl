use std::marker::PhantomPinned;

use crate::sdl;
use crate::sys;

pub enum Event<UserEvent = ()> {
    Active(ActiveEvent),
    Keyboard(KeyboardEvent),
    MouseMotion(MouseMotionEvent),
    MouseButton(MouseButtonEvent),
    JoyAxis(JoyAxisEvent),
    JoyButton(JoyButtonEvent),
    JoyHat(JoyHatEvent),
    JoyBall(JoyBallEvent),
    Resize(ResizeEvent),
    Expose,
    SysWM, // TODO: decide how/if we want to support this
    Quit,
    User(UserEvent),
    Unknown,
}

pub enum ActiveEvent {
    MouseEnter,
    MouseLeave,
    AppFocused,
    AppUnfocused,
    Minimized,
    Restored,
    Unknown,
}

impl From<sys::SDL_ActiveEvent> for ActiveEvent {
    fn from(value: sys::SDL_ActiveEvent) -> Self {
        match (value.state, value.gain) {
            (sys::SDL_APPMOUSEFOCUS, 0) => ActiveEvent::MouseLeave,
            (sys::SDL_APPMOUSEFOCUS, 1) => ActiveEvent::MouseEnter,
            (sys::SDL_APPINPUTFOCUS, 0) => ActiveEvent::AppUnfocused,
            (sys::SDL_APPINPUTFOCUS, 1) => ActiveEvent::AppFocused,
            (sys::SDL_APPACTIVE, 0) => ActiveEvent::Minimized,
            (sys::SDL_APPACTIVE, 1) => ActiveEvent::Restored,
            (_, _) => ActiveEvent::Unknown,
        }
    }
}

impl From<ActiveEvent> for Event {
    fn from(value: ActiveEvent) -> Self {
        Event::Active(value)
    }
}

pub enum KeyboardEvent {
    KeyUp(sys::SDL_keysym),
    KeyDown(sys::SDL_keysym),
}

pub struct MouseMotionEvent {
    pub x: u16,
    pub y: u16,
    pub xrel: u16,
    pub yrel: u16,

    // This event also contains a bitmask representing the current pressed
    // buttons, but it's incomplete and only supports 8 buttons, so we don't
    // support it. You should instead use the MouseButtonEvent.
    //
    // pub button_state: ButtonStateBitmask,
}

#[repr(u8)]
pub enum ButtonState {
    Pressed = sys::SDL_PRESSED,
    Released = sys::SDL_RELEASED,
}

#[repr(u8)]
pub enum Button {
    Left = sys::SDL_BUTTON_LEFT,
    Middle = sys::SDL_BUTTON_MIDDLE,
    Right = sys::SDL_BUTTON_RIGHT,
    WheelUp = sys::SDL_BUTTON_WHEELUP,
    WheelDown = sys::SDL_BUTTON_WHEELDOWN,
    X1 = sys::SDL_BUTTON_X1,
    X2 = sys::SDL_BUTTON_X2,
    Other(u8),
}

pub struct MouseButtonEvent {
    pub button: Button,
    pub pressed: bool,
    pub x: u8,
    pub y: u8,
}

pub struct JoyAxisEvent {
    pub device: u8,
    pub axis: u8,
    pub value: i16,
}

pub struct JoyButtonEvent {
    // NOTE: shows up as both SDL_JOYBUTTONDOWN and SDL_JOYBUTTONUP
    pub device: u8,
    pub button: u8,
    pub pressed: bool,
}

pub struct JoyHatEvent {
    pub device: u8,
    pub hat: u8,
    pub value: u8, // TODO: this has defines which may be helpful
}

pub struct JoyBallEvent {
    pub device: u8,
    pub ball: u8,
    pub xrel: i16,
    pub yrel: i16,
}

pub struct ResizeEvent {
    pub w: i32,
    pub h: i32,
}

#[derive(Debug)]
pub struct Subsystem {
    _pinned: std::marker::PhantomPinned,
}

impl Drop for Subsystem {
    fn drop(&mut self) {
        unsafe { sys::SDL_QuitSubSystem(sys::SDL_INIT_EVENTTHREAD) }
    }
}

impl Subsystem {
    pub(crate) fn new() -> sdl::Result<Subsystem> {
        if unsafe { sys::SDL_InitSubSystem(sys::SDL_INIT_EVENTTHREAD) } != 0 {
            Err(sdl::get_error())
        } else {
            Ok(Subsystem {
                _pinned: PhantomPinned,
            })
        }
    }
}
