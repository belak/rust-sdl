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

macro_rules! event_from {
    ($i:ident, $t:ty, $sdlt:ty) => {
        impl<T> From<$t> for Event<T> {
            fn from(value: $t) -> Self {
                Event::$i(value)
            }
        }

        impl<T> From<$sdlt> for Event<T> {
            fn from(value: $sdlt) -> Self {
                Event::$i(value.into())
            }
        }
    };
}

impl From<sys::SDL_ExposeEvent> for Event {
    fn from(_value: sys::SDL_ExposeEvent) -> Self {
        Event::Expose
    }
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

event_from!(Active, ActiveEvent, sys::SDL_ActiveEvent);

pub enum KeyboardEvent {
    KeyUp(sys::SDL_keysym),
    KeyDown(sys::SDL_keysym),
    Unknown,
}

impl From<sys::SDL_KeyboardEvent> for KeyboardEvent {
    fn from(value: sys::SDL_KeyboardEvent) -> Self {
        match value.state {
            sys::SDL_RELEASED => KeyboardEvent::KeyUp(value.keysym),
            sys::SDL_PRESSED => KeyboardEvent::KeyDown(value.keysym),
            _ => KeyboardEvent::Unknown,
        }
    }
}

event_from!(Keyboard, KeyboardEvent, sys::SDL_KeyboardEvent);

pub struct MouseMotionEvent {
    pub x: u16,
    pub y: u16,
    pub xrel: i16,
    pub yrel: i16,
    // This event also contains a bitmask representing the current pressed
    // buttons, but it's incomplete and only supports 8 buttons, so we don't
    // support it. You should instead use the MouseButtonEvent.
    //
    // pub button_state: ButtonStateBitmask,
}

impl From<sys::SDL_MouseMotionEvent> for MouseMotionEvent {
    fn from(value: sys::SDL_MouseMotionEvent) -> Self {
        MouseMotionEvent {
            x: value.x,
            y: value.y,
            xrel: value.xrel,
            yrel: value.yrel,
        }
    }
}

event_from!(MouseMotion, MouseMotionEvent, sys::SDL_MouseMotionEvent);

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

impl From<u8> for Button {
    fn from(value: u8) -> Self {
        match value {
            sys::SDL_BUTTON_LEFT => Button::Left,
            sys::SDL_BUTTON_MIDDLE => Button::Middle,
            sys::SDL_BUTTON_RIGHT => Button::Right,
            sys::SDL_BUTTON_WHEELUP => Button::WheelUp,
            sys::SDL_BUTTON_WHEELDOWN => Button::WheelDown,
            sys::SDL_BUTTON_X1 => Button::X1,
            sys::SDL_BUTTON_X2 => Button::X2,
            _ => Button::Other(value),
        }
    }
}

pub struct MouseButtonEvent {
    pub button: Button,
    pub pressed: bool,
    pub x: u16,
    pub y: u16,
}

// TODO: impl from

impl From<sys::SDL_MouseButtonEvent> for MouseButtonEvent {
    fn from(value: sys::SDL_MouseButtonEvent) -> Self {
        MouseButtonEvent {
            button: value.button.into(),
            pressed: value.state == sys::SDL_PRESSED,
            x: value.x,
            y: value.y,
        }
    }
}

event_from!(MouseButton, MouseButtonEvent, sys::SDL_MouseButtonEvent);

pub struct JoyAxisEvent {
    pub device: u8,
    pub axis: u8,
    pub value: i16,
}

impl From<sys::SDL_JoyAxisEvent> for JoyAxisEvent {
    fn from(value: sys::SDL_JoyAxisEvent) -> Self {
        JoyAxisEvent {
            device: value.which,
            axis: value.axis,
            value: value.value,
        }
    }
}

event_from!(JoyAxis, JoyAxisEvent, sys::SDL_JoyAxisEvent);

pub struct JoyButtonEvent {
    // NOTE: shows up as both SDL_JOYBUTTONDOWN and SDL_JOYBUTTONUP
    pub device: u8,
    pub button: u8,
    pub pressed: bool,
}

impl From<sys::SDL_JoyButtonEvent> for JoyButtonEvent {
    fn from(value: sys::SDL_JoyButtonEvent) -> Self {
        JoyButtonEvent {
            device: value.which,
            button: value.button,
            pressed: value.state == sys::SDL_PRESSED,
        }
    }
}

event_from!(JoyButton, JoyButtonEvent, sys::SDL_JoyButtonEvent);

pub struct JoyHatEvent {
    pub device: u8,
    pub hat: u8,
    pub value: u8, // TODO: this has defines which may be helpful
}

impl From<sys::SDL_JoyHatEvent> for JoyHatEvent {
    fn from(value: sys::SDL_JoyHatEvent) -> Self {
        JoyHatEvent {
            device: value.which,
            hat: value.hat,
            value: value.value,
        }
    }
}

event_from!(JoyHat, JoyHatEvent, sys::SDL_JoyHatEvent);

pub struct JoyBallEvent {
    pub device: u8,
    pub ball: u8,
    pub xrel: i16,
    pub yrel: i16,
}

impl From<sys::SDL_JoyBallEvent> for JoyBallEvent {
    fn from(value: sys::SDL_JoyBallEvent) -> Self {
        JoyBallEvent {
            device: value.which,
            ball: value.ball,
            xrel: value.xrel,
            yrel: value.yrel,
        }
    }
}

event_from!(JoyBall, JoyBallEvent, sys::SDL_JoyBallEvent);

pub struct ResizeEvent {
    pub w: i32,
    pub h: i32,
}

impl From<sys::SDL_ResizeEvent> for ResizeEvent {
    fn from(value: sys::SDL_ResizeEvent) -> Self {
        ResizeEvent {
            w: value.w,
            h: value.h,
        }
    }
}

event_from!(Resize, ResizeEvent, sys::SDL_ResizeEvent);

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
