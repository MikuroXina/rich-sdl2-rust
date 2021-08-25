//! Virtual key codes corresponded to physical scan codes.

#![allow(missing_docs)]

use std::ffi::{CStr, CString};

use crate::bind;

use super::scan_code::ScanCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    Unknown,
    Return,
    Escape,
    Backspace,
    Tab,
    Space,
    Exclaim,
    Quotedbl,
    Hash,
    Percent,
    Dollar,
    Ampersand,
    Quote,
    Leftparen,
    Rightparen,
    Asterisk,
    Plus,
    Comma,
    Minus,
    Period,
    Slash,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Colon,
    Semicolon,
    Less,
    Equals,
    Greater,
    Question,
    At,
    LeftBracket,
    Backslash,
    RightBracket,
    Caret,
    Underscore,
    Backquote,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    CapsLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,
    Right,
    Left,
    Down,
    Up,
    NumLockClear,
    KeypadDivide,
    KeypadMultiply,
    KeypadMinus,
    KeypadPlus,
    KeypadEnter,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    Keypad0,
    KeypadPeriod,
    Application,
    Power,
    KeypadEquals,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Execute,
    Help,
    Menu,
    Select,
    Stop,
    Again,
    Undo,
    Cut,
    Copy,
    Paste,
    Find,
    Mute,
    VolumeUp,
    VolumeDown,
    KeypadComma,
    KeypadEqualsAs400,
    Alterase,
    SysReq,
    Cancel,
    Clear,
    Prior,
    Return2,
    Separator,
    Out,
    Oper,
    ClearAgain,
    CrSel,
    ExSel,
    Keypad00,
    Keypad000,
    ThousandsSeparator,
    DecimalSeparator,
    CurrencyUnit,
    CurrencySubunit,
    KeypadLeftParen,
    KeypadRightParen,
    KeypadLeftBrace,
    KeypadRightBrace,
    KeypadTab,
    KeypadBackspace,
    KeypadA,
    KeypadB,
    KeypadC,
    KeypadD,
    KeypadE,
    KeypadF,
    KeypadXor,
    KeypadPower,
    KeypadPercent,
    KeypadLess,
    KeypadGreater,
    KeypadAmpersand,
    KeypadDblAmpersand,
    KeypadVerticalBar,
    KeypadDblverticalBar,
    KeypadColon,
    KeypadHash,
    KeypadSpace,
    KeypadAt,
    KeypadExclam,
    KeypadMemStore,
    KeypadMemRecall,
    KeypadMemClear,
    KeypadMemAdd,
    KeypadMemSubtract,
    KeypadMemMultiply,
    KeypadMemDivide,
    KeypadPlusMinus,
    KeypadClear,
    KeypadClearEntry,
    KeypadBinary,
    KeypadOctal,
    KeypadDecimal,
    KeypadHexadecimal,
    LCtrl,
    LShift,
    LAlt,
    LGui,
    RCtrl,
    RShift,
    RAlt,
    RGui,
    Mode,
    AudioNext,
    AudioPrev,
    AudioStop,
    AudioPlay,
    AudioMute,
    MediaSelect,
    Www,
    Mail,
    Calculator,
    Computer,
    ApplicationSearch,
    ApplicationHome,
    ApplicationBack,
    ApplicationForward,
    ApplicationStop,
    ApplicationRefresh,
    ApplicationBookmarks,
    BrightnessDown,
    BrightnessUp,
    DisplaySwitch,
    KeyboardIllumToggle,
    KeyboardIllumDown,
    KeyboardIllumUp,
    Eject,
    Sleep,
    App1,
    App2,
    AudioRewind,
    AudioFastForward,
}

impl KeyCode {
    pub fn from_name(name: &str) -> Self {
        let c_str = CString::new(name).expect("name must be a valid string");
        unsafe { bind::SDL_GetKeyFromName(c_str.as_ptr()) }.into()
    }

    pub fn name(&self) -> Option<String> {
        let name: String = unsafe { CStr::from_ptr(bind::SDL_GetKeyName(self.as_raw())) }
            .to_str()
            .unwrap()
            .into();
        (!name.is_empty()).then(|| name)
    }

    pub(crate) fn as_raw(&self) -> bind::SDL_Keycode {
        use KeyCode::*;
        (match self {
            Unknown => bind::SDL_KeyCode_SDLK_UNKNOWN,
            Return => bind::SDL_KeyCode_SDLK_RETURN,
            Escape => bind::SDL_KeyCode_SDLK_ESCAPE,
            Backspace => bind::SDL_KeyCode_SDLK_BACKSPACE,
            Tab => bind::SDL_KeyCode_SDLK_TAB,
            Space => bind::SDL_KeyCode_SDLK_SPACE,
            Exclaim => bind::SDL_KeyCode_SDLK_EXCLAIM,
            Quotedbl => bind::SDL_KeyCode_SDLK_QUOTEDBL,
            Hash => bind::SDL_KeyCode_SDLK_HASH,
            Percent => bind::SDL_KeyCode_SDLK_PERCENT,
            Dollar => bind::SDL_KeyCode_SDLK_DOLLAR,
            Ampersand => bind::SDL_KeyCode_SDLK_AMPERSAND,
            Quote => bind::SDL_KeyCode_SDLK_QUOTE,
            Leftparen => bind::SDL_KeyCode_SDLK_LEFTPAREN,
            Rightparen => bind::SDL_KeyCode_SDLK_RIGHTPAREN,
            Asterisk => bind::SDL_KeyCode_SDLK_ASTERISK,
            Plus => bind::SDL_KeyCode_SDLK_PLUS,
            Comma => bind::SDL_KeyCode_SDLK_COMMA,
            Minus => bind::SDL_KeyCode_SDLK_MINUS,
            Period => bind::SDL_KeyCode_SDLK_PERIOD,
            Slash => bind::SDL_KeyCode_SDLK_SLASH,
            Num0 => bind::SDL_KeyCode_SDLK_0,
            Num1 => bind::SDL_KeyCode_SDLK_1,
            Num2 => bind::SDL_KeyCode_SDLK_2,
            Num3 => bind::SDL_KeyCode_SDLK_3,
            Num4 => bind::SDL_KeyCode_SDLK_4,
            Num5 => bind::SDL_KeyCode_SDLK_5,
            Num6 => bind::SDL_KeyCode_SDLK_6,
            Num7 => bind::SDL_KeyCode_SDLK_7,
            Num8 => bind::SDL_KeyCode_SDLK_8,
            Num9 => bind::SDL_KeyCode_SDLK_9,
            Colon => bind::SDL_KeyCode_SDLK_COLON,
            Semicolon => bind::SDL_KeyCode_SDLK_SEMICOLON,
            Less => bind::SDL_KeyCode_SDLK_LESS,
            Equals => bind::SDL_KeyCode_SDLK_EQUALS,
            Greater => bind::SDL_KeyCode_SDLK_GREATER,
            Question => bind::SDL_KeyCode_SDLK_QUESTION,
            At => bind::SDL_KeyCode_SDLK_AT,
            LeftBracket => bind::SDL_KeyCode_SDLK_LEFTBRACKET,
            Backslash => bind::SDL_KeyCode_SDLK_BACKSLASH,
            RightBracket => bind::SDL_KeyCode_SDLK_RIGHTBRACKET,
            Caret => bind::SDL_KeyCode_SDLK_CARET,
            Underscore => bind::SDL_KeyCode_SDLK_UNDERSCORE,
            Backquote => bind::SDL_KeyCode_SDLK_BACKQUOTE,
            A => bind::SDL_KeyCode_SDLK_a,
            B => bind::SDL_KeyCode_SDLK_b,
            C => bind::SDL_KeyCode_SDLK_c,
            D => bind::SDL_KeyCode_SDLK_d,
            E => bind::SDL_KeyCode_SDLK_e,
            F => bind::SDL_KeyCode_SDLK_f,
            G => bind::SDL_KeyCode_SDLK_g,
            H => bind::SDL_KeyCode_SDLK_h,
            I => bind::SDL_KeyCode_SDLK_i,
            J => bind::SDL_KeyCode_SDLK_j,
            K => bind::SDL_KeyCode_SDLK_k,
            L => bind::SDL_KeyCode_SDLK_l,
            M => bind::SDL_KeyCode_SDLK_m,
            N => bind::SDL_KeyCode_SDLK_n,
            O => bind::SDL_KeyCode_SDLK_o,
            P => bind::SDL_KeyCode_SDLK_p,
            Q => bind::SDL_KeyCode_SDLK_q,
            R => bind::SDL_KeyCode_SDLK_r,
            S => bind::SDL_KeyCode_SDLK_s,
            T => bind::SDL_KeyCode_SDLK_t,
            U => bind::SDL_KeyCode_SDLK_u,
            V => bind::SDL_KeyCode_SDLK_v,
            W => bind::SDL_KeyCode_SDLK_w,
            X => bind::SDL_KeyCode_SDLK_x,
            Y => bind::SDL_KeyCode_SDLK_y,
            Z => bind::SDL_KeyCode_SDLK_z,
            CapsLock => bind::SDL_KeyCode_SDLK_CAPSLOCK,
            F1 => bind::SDL_KeyCode_SDLK_F1,
            F2 => bind::SDL_KeyCode_SDLK_F2,
            F3 => bind::SDL_KeyCode_SDLK_F3,
            F4 => bind::SDL_KeyCode_SDLK_F4,
            F5 => bind::SDL_KeyCode_SDLK_F5,
            F6 => bind::SDL_KeyCode_SDLK_F6,
            F7 => bind::SDL_KeyCode_SDLK_F7,
            F8 => bind::SDL_KeyCode_SDLK_F8,
            F9 => bind::SDL_KeyCode_SDLK_F9,
            F10 => bind::SDL_KeyCode_SDLK_F10,
            F11 => bind::SDL_KeyCode_SDLK_F11,
            F12 => bind::SDL_KeyCode_SDLK_F12,
            PrintScreen => bind::SDL_KeyCode_SDLK_PRINTSCREEN,
            ScrollLock => bind::SDL_KeyCode_SDLK_SCROLLLOCK,
            Pause => bind::SDL_KeyCode_SDLK_PAUSE,
            Insert => bind::SDL_KeyCode_SDLK_INSERT,
            Home => bind::SDL_KeyCode_SDLK_HOME,
            PageUp => bind::SDL_KeyCode_SDLK_PAGEUP,
            Delete => bind::SDL_KeyCode_SDLK_DELETE,
            End => bind::SDL_KeyCode_SDLK_END,
            PageDown => bind::SDL_KeyCode_SDLK_PAGEDOWN,
            Right => bind::SDL_KeyCode_SDLK_RIGHT,
            Left => bind::SDL_KeyCode_SDLK_LEFT,
            Down => bind::SDL_KeyCode_SDLK_DOWN,
            Up => bind::SDL_KeyCode_SDLK_UP,
            NumLockClear => bind::SDL_KeyCode_SDLK_NUMLOCKCLEAR,
            KeypadDivide => bind::SDL_KeyCode_SDLK_KP_DIVIDE,
            KeypadMultiply => bind::SDL_KeyCode_SDLK_KP_MULTIPLY,
            KeypadMinus => bind::SDL_KeyCode_SDLK_KP_MINUS,
            KeypadPlus => bind::SDL_KeyCode_SDLK_KP_PLUS,
            KeypadEnter => bind::SDL_KeyCode_SDLK_KP_ENTER,
            Keypad1 => bind::SDL_KeyCode_SDLK_KP_1,
            Keypad2 => bind::SDL_KeyCode_SDLK_KP_2,
            Keypad3 => bind::SDL_KeyCode_SDLK_KP_3,
            Keypad4 => bind::SDL_KeyCode_SDLK_KP_4,
            Keypad5 => bind::SDL_KeyCode_SDLK_KP_5,
            Keypad6 => bind::SDL_KeyCode_SDLK_KP_6,
            Keypad7 => bind::SDL_KeyCode_SDLK_KP_7,
            Keypad8 => bind::SDL_KeyCode_SDLK_KP_8,
            Keypad9 => bind::SDL_KeyCode_SDLK_KP_9,
            Keypad0 => bind::SDL_KeyCode_SDLK_KP_0,
            KeypadPeriod => bind::SDL_KeyCode_SDLK_KP_PERIOD,
            Application => bind::SDL_KeyCode_SDLK_APPLICATION,
            Power => bind::SDL_KeyCode_SDLK_POWER,
            KeypadEquals => bind::SDL_KeyCode_SDLK_KP_EQUALS,
            F13 => bind::SDL_KeyCode_SDLK_F13,
            F14 => bind::SDL_KeyCode_SDLK_F14,
            F15 => bind::SDL_KeyCode_SDLK_F15,
            F16 => bind::SDL_KeyCode_SDLK_F16,
            F17 => bind::SDL_KeyCode_SDLK_F17,
            F18 => bind::SDL_KeyCode_SDLK_F18,
            F19 => bind::SDL_KeyCode_SDLK_F19,
            F20 => bind::SDL_KeyCode_SDLK_F20,
            F21 => bind::SDL_KeyCode_SDLK_F21,
            F22 => bind::SDL_KeyCode_SDLK_F22,
            F23 => bind::SDL_KeyCode_SDLK_F23,
            F24 => bind::SDL_KeyCode_SDLK_F24,
            Execute => bind::SDL_KeyCode_SDLK_EXECUTE,
            Help => bind::SDL_KeyCode_SDLK_HELP,
            Menu => bind::SDL_KeyCode_SDLK_MENU,
            Select => bind::SDL_KeyCode_SDLK_SELECT,
            Stop => bind::SDL_KeyCode_SDLK_STOP,
            Again => bind::SDL_KeyCode_SDLK_AGAIN,
            Undo => bind::SDL_KeyCode_SDLK_UNDO,
            Cut => bind::SDL_KeyCode_SDLK_CUT,
            Copy => bind::SDL_KeyCode_SDLK_COPY,
            Paste => bind::SDL_KeyCode_SDLK_PASTE,
            Find => bind::SDL_KeyCode_SDLK_FIND,
            Mute => bind::SDL_KeyCode_SDLK_MUTE,
            VolumeUp => bind::SDL_KeyCode_SDLK_VOLUMEUP,
            VolumeDown => bind::SDL_KeyCode_SDLK_VOLUMEDOWN,
            KeypadComma => bind::SDL_KeyCode_SDLK_KP_COMMA,
            KeypadEqualsAs400 => bind::SDL_KeyCode_SDLK_KP_EQUALSAS400,
            Alterase => bind::SDL_KeyCode_SDLK_ALTERASE,
            SysReq => bind::SDL_KeyCode_SDLK_SYSREQ,
            Cancel => bind::SDL_KeyCode_SDLK_CANCEL,
            Clear => bind::SDL_KeyCode_SDLK_CLEAR,
            Prior => bind::SDL_KeyCode_SDLK_PRIOR,
            Return2 => bind::SDL_KeyCode_SDLK_RETURN2,
            Separator => bind::SDL_KeyCode_SDLK_SEPARATOR,
            Out => bind::SDL_KeyCode_SDLK_OUT,
            Oper => bind::SDL_KeyCode_SDLK_OPER,
            ClearAgain => bind::SDL_KeyCode_SDLK_CLEARAGAIN,
            CrSel => bind::SDL_KeyCode_SDLK_CRSEL,
            ExSel => bind::SDL_KeyCode_SDLK_EXSEL,
            Keypad00 => bind::SDL_KeyCode_SDLK_KP_00,
            Keypad000 => bind::SDL_KeyCode_SDLK_KP_000,
            ThousandsSeparator => bind::SDL_KeyCode_SDLK_THOUSANDSSEPARATOR,
            DecimalSeparator => bind::SDL_KeyCode_SDLK_DECIMALSEPARATOR,
            CurrencyUnit => bind::SDL_KeyCode_SDLK_CURRENCYUNIT,
            CurrencySubunit => bind::SDL_KeyCode_SDLK_CURRENCYSUBUNIT,
            KeypadLeftParen => bind::SDL_KeyCode_SDLK_KP_LEFTPAREN,
            KeypadRightParen => bind::SDL_KeyCode_SDLK_KP_RIGHTPAREN,
            KeypadLeftBrace => bind::SDL_KeyCode_SDLK_KP_LEFTBRACE,
            KeypadRightBrace => bind::SDL_KeyCode_SDLK_KP_RIGHTBRACE,
            KeypadTab => bind::SDL_KeyCode_SDLK_KP_TAB,
            KeypadBackspace => bind::SDL_KeyCode_SDLK_KP_BACKSPACE,
            KeypadA => bind::SDL_KeyCode_SDLK_KP_A,
            KeypadB => bind::SDL_KeyCode_SDLK_KP_B,
            KeypadC => bind::SDL_KeyCode_SDLK_KP_C,
            KeypadD => bind::SDL_KeyCode_SDLK_KP_D,
            KeypadE => bind::SDL_KeyCode_SDLK_KP_E,
            KeypadF => bind::SDL_KeyCode_SDLK_KP_F,
            KeypadXor => bind::SDL_KeyCode_SDLK_KP_XOR,
            KeypadPower => bind::SDL_KeyCode_SDLK_KP_POWER,
            KeypadPercent => bind::SDL_KeyCode_SDLK_KP_PERCENT,
            KeypadLess => bind::SDL_KeyCode_SDLK_KP_LESS,
            KeypadGreater => bind::SDL_KeyCode_SDLK_KP_GREATER,
            KeypadAmpersand => bind::SDL_KeyCode_SDLK_KP_AMPERSAND,
            KeypadDblAmpersand => bind::SDL_KeyCode_SDLK_KP_DBLAMPERSAND,
            KeypadVerticalBar => bind::SDL_KeyCode_SDLK_KP_VERTICALBAR,
            KeypadDblverticalBar => bind::SDL_KeyCode_SDLK_KP_DBLVERTICALBAR,
            KeypadColon => bind::SDL_KeyCode_SDLK_KP_COLON,
            KeypadHash => bind::SDL_KeyCode_SDLK_KP_HASH,
            KeypadSpace => bind::SDL_KeyCode_SDLK_KP_SPACE,
            KeypadAt => bind::SDL_KeyCode_SDLK_KP_AT,
            KeypadExclam => bind::SDL_KeyCode_SDLK_KP_EXCLAM,
            KeypadMemStore => bind::SDL_KeyCode_SDLK_KP_MEMSTORE,
            KeypadMemRecall => bind::SDL_KeyCode_SDLK_KP_MEMRECALL,
            KeypadMemClear => bind::SDL_KeyCode_SDLK_KP_MEMCLEAR,
            KeypadMemAdd => bind::SDL_KeyCode_SDLK_KP_MEMADD,
            KeypadMemSubtract => bind::SDL_KeyCode_SDLK_KP_MEMSUBTRACT,
            KeypadMemMultiply => bind::SDL_KeyCode_SDLK_KP_MEMMULTIPLY,
            KeypadMemDivide => bind::SDL_KeyCode_SDLK_KP_MEMDIVIDE,
            KeypadPlusMinus => bind::SDL_KeyCode_SDLK_KP_PLUSMINUS,
            KeypadClear => bind::SDL_KeyCode_SDLK_KP_CLEAR,
            KeypadClearEntry => bind::SDL_KeyCode_SDLK_KP_CLEARENTRY,
            KeypadBinary => bind::SDL_KeyCode_SDLK_KP_BINARY,
            KeypadOctal => bind::SDL_KeyCode_SDLK_KP_OCTAL,
            KeypadDecimal => bind::SDL_KeyCode_SDLK_KP_DECIMAL,
            KeypadHexadecimal => bind::SDL_KeyCode_SDLK_KP_HEXADECIMAL,
            LCtrl => bind::SDL_KeyCode_SDLK_LCTRL,
            LShift => bind::SDL_KeyCode_SDLK_LSHIFT,
            LAlt => bind::SDL_KeyCode_SDLK_LALT,
            LGui => bind::SDL_KeyCode_SDLK_LGUI,
            RCtrl => bind::SDL_KeyCode_SDLK_RCTRL,
            RShift => bind::SDL_KeyCode_SDLK_RSHIFT,
            RAlt => bind::SDL_KeyCode_SDLK_RALT,
            RGui => bind::SDL_KeyCode_SDLK_RGUI,
            Mode => bind::SDL_KeyCode_SDLK_MODE,
            AudioNext => bind::SDL_KeyCode_SDLK_AUDIONEXT,
            AudioPrev => bind::SDL_KeyCode_SDLK_AUDIOPREV,
            AudioStop => bind::SDL_KeyCode_SDLK_AUDIOSTOP,
            AudioPlay => bind::SDL_KeyCode_SDLK_AUDIOPLAY,
            AudioMute => bind::SDL_KeyCode_SDLK_AUDIOMUTE,
            MediaSelect => bind::SDL_KeyCode_SDLK_MEDIASELECT,
            Www => bind::SDL_KeyCode_SDLK_WWW,
            Mail => bind::SDL_KeyCode_SDLK_MAIL,
            Calculator => bind::SDL_KeyCode_SDLK_CALCULATOR,
            Computer => bind::SDL_KeyCode_SDLK_COMPUTER,
            ApplicationSearch => bind::SDL_KeyCode_SDLK_AC_SEARCH,
            ApplicationHome => bind::SDL_KeyCode_SDLK_AC_HOME,
            ApplicationBack => bind::SDL_KeyCode_SDLK_AC_BACK,
            ApplicationForward => bind::SDL_KeyCode_SDLK_AC_FORWARD,
            ApplicationStop => bind::SDL_KeyCode_SDLK_AC_STOP,
            ApplicationRefresh => bind::SDL_KeyCode_SDLK_AC_REFRESH,
            ApplicationBookmarks => bind::SDL_KeyCode_SDLK_AC_BOOKMARKS,
            BrightnessDown => bind::SDL_KeyCode_SDLK_BRIGHTNESSDOWN,
            BrightnessUp => bind::SDL_KeyCode_SDLK_BRIGHTNESSUP,
            DisplaySwitch => bind::SDL_KeyCode_SDLK_DISPLAYSWITCH,
            KeyboardIllumToggle => bind::SDL_KeyCode_SDLK_KBDILLUMTOGGLE,
            KeyboardIllumDown => bind::SDL_KeyCode_SDLK_KBDILLUMDOWN,
            KeyboardIllumUp => bind::SDL_KeyCode_SDLK_KBDILLUMUP,
            Eject => bind::SDL_KeyCode_SDLK_EJECT,
            Sleep => bind::SDL_KeyCode_SDLK_SLEEP,
            App1 => bind::SDL_KeyCode_SDLK_APP1,
            App2 => bind::SDL_KeyCode_SDLK_APP2,
            AudioRewind => bind::SDL_KeyCode_SDLK_AUDIOREWIND,
            AudioFastForward => bind::SDL_KeyCode_SDLK_AUDIOFASTFORWARD,
        }) as bind::SDL_Keycode
    }
}

impl From<ScanCode> for KeyCode {
    fn from(scan_code: ScanCode) -> Self {
        let raw = scan_code.as_raw();
        unsafe { bind::SDL_GetKeyFromScancode(raw) }.into()
    }
}

impl From<bind::SDL_Keycode> for KeyCode {
    fn from(code: bind::SDL_Keycode) -> Self {
        use KeyCode::*;
        match code as u32 {
            bind::SDL_KeyCode_SDLK_RETURN => Return,
            bind::SDL_KeyCode_SDLK_ESCAPE => Escape,
            bind::SDL_KeyCode_SDLK_BACKSPACE => Backspace,
            bind::SDL_KeyCode_SDLK_TAB => Tab,
            bind::SDL_KeyCode_SDLK_SPACE => Space,
            bind::SDL_KeyCode_SDLK_EXCLAIM => Exclaim,
            bind::SDL_KeyCode_SDLK_QUOTEDBL => Quotedbl,
            bind::SDL_KeyCode_SDLK_HASH => Hash,
            bind::SDL_KeyCode_SDLK_PERCENT => Percent,
            bind::SDL_KeyCode_SDLK_DOLLAR => Dollar,
            bind::SDL_KeyCode_SDLK_AMPERSAND => Ampersand,
            bind::SDL_KeyCode_SDLK_QUOTE => Quote,
            bind::SDL_KeyCode_SDLK_LEFTPAREN => Leftparen,
            bind::SDL_KeyCode_SDLK_RIGHTPAREN => Rightparen,
            bind::SDL_KeyCode_SDLK_ASTERISK => Asterisk,
            bind::SDL_KeyCode_SDLK_PLUS => Plus,
            bind::SDL_KeyCode_SDLK_COMMA => Comma,
            bind::SDL_KeyCode_SDLK_MINUS => Minus,
            bind::SDL_KeyCode_SDLK_PERIOD => Period,
            bind::SDL_KeyCode_SDLK_SLASH => Slash,
            bind::SDL_KeyCode_SDLK_0 => Num0,
            bind::SDL_KeyCode_SDLK_1 => Num1,
            bind::SDL_KeyCode_SDLK_2 => Num2,
            bind::SDL_KeyCode_SDLK_3 => Num3,
            bind::SDL_KeyCode_SDLK_4 => Num4,
            bind::SDL_KeyCode_SDLK_5 => Num5,
            bind::SDL_KeyCode_SDLK_6 => Num6,
            bind::SDL_KeyCode_SDLK_7 => Num7,
            bind::SDL_KeyCode_SDLK_8 => Num8,
            bind::SDL_KeyCode_SDLK_9 => Num9,
            bind::SDL_KeyCode_SDLK_COLON => Colon,
            bind::SDL_KeyCode_SDLK_SEMICOLON => Semicolon,
            bind::SDL_KeyCode_SDLK_LESS => Less,
            bind::SDL_KeyCode_SDLK_EQUALS => Equals,
            bind::SDL_KeyCode_SDLK_GREATER => Greater,
            bind::SDL_KeyCode_SDLK_QUESTION => Question,
            bind::SDL_KeyCode_SDLK_AT => At,
            bind::SDL_KeyCode_SDLK_LEFTBRACKET => LeftBracket,
            bind::SDL_KeyCode_SDLK_BACKSLASH => Backslash,
            bind::SDL_KeyCode_SDLK_RIGHTBRACKET => RightBracket,
            bind::SDL_KeyCode_SDLK_CARET => Caret,
            bind::SDL_KeyCode_SDLK_UNDERSCORE => Underscore,
            bind::SDL_KeyCode_SDLK_BACKQUOTE => Backquote,
            bind::SDL_KeyCode_SDLK_a => A,
            bind::SDL_KeyCode_SDLK_b => B,
            bind::SDL_KeyCode_SDLK_c => C,
            bind::SDL_KeyCode_SDLK_d => D,
            bind::SDL_KeyCode_SDLK_e => E,
            bind::SDL_KeyCode_SDLK_f => F,
            bind::SDL_KeyCode_SDLK_g => G,
            bind::SDL_KeyCode_SDLK_h => H,
            bind::SDL_KeyCode_SDLK_i => I,
            bind::SDL_KeyCode_SDLK_j => J,
            bind::SDL_KeyCode_SDLK_k => K,
            bind::SDL_KeyCode_SDLK_l => L,
            bind::SDL_KeyCode_SDLK_m => M,
            bind::SDL_KeyCode_SDLK_n => N,
            bind::SDL_KeyCode_SDLK_o => O,
            bind::SDL_KeyCode_SDLK_p => P,
            bind::SDL_KeyCode_SDLK_q => Q,
            bind::SDL_KeyCode_SDLK_r => R,
            bind::SDL_KeyCode_SDLK_s => S,
            bind::SDL_KeyCode_SDLK_t => T,
            bind::SDL_KeyCode_SDLK_u => U,
            bind::SDL_KeyCode_SDLK_v => V,
            bind::SDL_KeyCode_SDLK_w => W,
            bind::SDL_KeyCode_SDLK_x => X,
            bind::SDL_KeyCode_SDLK_y => Y,
            bind::SDL_KeyCode_SDLK_z => Z,
            bind::SDL_KeyCode_SDLK_CAPSLOCK => CapsLock,
            bind::SDL_KeyCode_SDLK_F1 => F1,
            bind::SDL_KeyCode_SDLK_F2 => F2,
            bind::SDL_KeyCode_SDLK_F3 => F3,
            bind::SDL_KeyCode_SDLK_F4 => F4,
            bind::SDL_KeyCode_SDLK_F5 => F5,
            bind::SDL_KeyCode_SDLK_F6 => F6,
            bind::SDL_KeyCode_SDLK_F7 => F7,
            bind::SDL_KeyCode_SDLK_F8 => F8,
            bind::SDL_KeyCode_SDLK_F9 => F9,
            bind::SDL_KeyCode_SDLK_F10 => F10,
            bind::SDL_KeyCode_SDLK_F11 => F11,
            bind::SDL_KeyCode_SDLK_F12 => F12,
            bind::SDL_KeyCode_SDLK_PRINTSCREEN => PrintScreen,
            bind::SDL_KeyCode_SDLK_SCROLLLOCK => ScrollLock,
            bind::SDL_KeyCode_SDLK_PAUSE => Pause,
            bind::SDL_KeyCode_SDLK_INSERT => Insert,
            bind::SDL_KeyCode_SDLK_HOME => Home,
            bind::SDL_KeyCode_SDLK_PAGEUP => PageUp,
            bind::SDL_KeyCode_SDLK_DELETE => Delete,
            bind::SDL_KeyCode_SDLK_END => End,
            bind::SDL_KeyCode_SDLK_PAGEDOWN => PageDown,
            bind::SDL_KeyCode_SDLK_RIGHT => Right,
            bind::SDL_KeyCode_SDLK_LEFT => Left,
            bind::SDL_KeyCode_SDLK_DOWN => Down,
            bind::SDL_KeyCode_SDLK_UP => Up,
            bind::SDL_KeyCode_SDLK_NUMLOCKCLEAR => NumLockClear,
            bind::SDL_KeyCode_SDLK_KP_DIVIDE => KeypadDivide,
            bind::SDL_KeyCode_SDLK_KP_MULTIPLY => KeypadMultiply,
            bind::SDL_KeyCode_SDLK_KP_MINUS => KeypadMinus,
            bind::SDL_KeyCode_SDLK_KP_PLUS => KeypadPlus,
            bind::SDL_KeyCode_SDLK_KP_ENTER => KeypadEnter,
            bind::SDL_KeyCode_SDLK_KP_1 => Keypad1,
            bind::SDL_KeyCode_SDLK_KP_2 => Keypad2,
            bind::SDL_KeyCode_SDLK_KP_3 => Keypad3,
            bind::SDL_KeyCode_SDLK_KP_4 => Keypad4,
            bind::SDL_KeyCode_SDLK_KP_5 => Keypad5,
            bind::SDL_KeyCode_SDLK_KP_6 => Keypad6,
            bind::SDL_KeyCode_SDLK_KP_7 => Keypad7,
            bind::SDL_KeyCode_SDLK_KP_8 => Keypad8,
            bind::SDL_KeyCode_SDLK_KP_9 => Keypad9,
            bind::SDL_KeyCode_SDLK_KP_0 => Keypad0,
            bind::SDL_KeyCode_SDLK_KP_PERIOD => KeypadPeriod,
            bind::SDL_KeyCode_SDLK_APPLICATION => Application,
            bind::SDL_KeyCode_SDLK_POWER => Power,
            bind::SDL_KeyCode_SDLK_KP_EQUALS => KeypadEquals,
            bind::SDL_KeyCode_SDLK_F13 => F13,
            bind::SDL_KeyCode_SDLK_F14 => F14,
            bind::SDL_KeyCode_SDLK_F15 => F15,
            bind::SDL_KeyCode_SDLK_F16 => F16,
            bind::SDL_KeyCode_SDLK_F17 => F17,
            bind::SDL_KeyCode_SDLK_F18 => F18,
            bind::SDL_KeyCode_SDLK_F19 => F19,
            bind::SDL_KeyCode_SDLK_F20 => F20,
            bind::SDL_KeyCode_SDLK_F21 => F21,
            bind::SDL_KeyCode_SDLK_F22 => F22,
            bind::SDL_KeyCode_SDLK_F23 => F23,
            bind::SDL_KeyCode_SDLK_F24 => F24,
            bind::SDL_KeyCode_SDLK_EXECUTE => Execute,
            bind::SDL_KeyCode_SDLK_HELP => Help,
            bind::SDL_KeyCode_SDLK_MENU => Menu,
            bind::SDL_KeyCode_SDLK_SELECT => Select,
            bind::SDL_KeyCode_SDLK_STOP => Stop,
            bind::SDL_KeyCode_SDLK_AGAIN => Again,
            bind::SDL_KeyCode_SDLK_UNDO => Undo,
            bind::SDL_KeyCode_SDLK_CUT => Cut,
            bind::SDL_KeyCode_SDLK_COPY => Copy,
            bind::SDL_KeyCode_SDLK_PASTE => Paste,
            bind::SDL_KeyCode_SDLK_FIND => Find,
            bind::SDL_KeyCode_SDLK_MUTE => Mute,
            bind::SDL_KeyCode_SDLK_VOLUMEUP => VolumeUp,
            bind::SDL_KeyCode_SDLK_VOLUMEDOWN => VolumeDown,
            bind::SDL_KeyCode_SDLK_KP_COMMA => KeypadComma,
            bind::SDL_KeyCode_SDLK_KP_EQUALSAS400 => KeypadEqualsAs400,
            bind::SDL_KeyCode_SDLK_ALTERASE => Alterase,
            bind::SDL_KeyCode_SDLK_SYSREQ => SysReq,
            bind::SDL_KeyCode_SDLK_CANCEL => Cancel,
            bind::SDL_KeyCode_SDLK_CLEAR => Clear,
            bind::SDL_KeyCode_SDLK_PRIOR => Prior,
            bind::SDL_KeyCode_SDLK_RETURN2 => Return2,
            bind::SDL_KeyCode_SDLK_SEPARATOR => Separator,
            bind::SDL_KeyCode_SDLK_OUT => Out,
            bind::SDL_KeyCode_SDLK_OPER => Oper,
            bind::SDL_KeyCode_SDLK_CLEARAGAIN => ClearAgain,
            bind::SDL_KeyCode_SDLK_CRSEL => CrSel,
            bind::SDL_KeyCode_SDLK_EXSEL => ExSel,
            bind::SDL_KeyCode_SDLK_KP_00 => Keypad00,
            bind::SDL_KeyCode_SDLK_KP_000 => Keypad000,
            bind::SDL_KeyCode_SDLK_THOUSANDSSEPARATOR => ThousandsSeparator,
            bind::SDL_KeyCode_SDLK_DECIMALSEPARATOR => DecimalSeparator,
            bind::SDL_KeyCode_SDLK_CURRENCYUNIT => CurrencyUnit,
            bind::SDL_KeyCode_SDLK_CURRENCYSUBUNIT => CurrencySubunit,
            bind::SDL_KeyCode_SDLK_KP_LEFTPAREN => KeypadLeftParen,
            bind::SDL_KeyCode_SDLK_KP_RIGHTPAREN => KeypadRightParen,
            bind::SDL_KeyCode_SDLK_KP_LEFTBRACE => KeypadLeftBrace,
            bind::SDL_KeyCode_SDLK_KP_RIGHTBRACE => KeypadRightBrace,
            bind::SDL_KeyCode_SDLK_KP_TAB => KeypadTab,
            bind::SDL_KeyCode_SDLK_KP_BACKSPACE => KeypadBackspace,
            bind::SDL_KeyCode_SDLK_KP_A => KeypadA,
            bind::SDL_KeyCode_SDLK_KP_B => KeypadB,
            bind::SDL_KeyCode_SDLK_KP_C => KeypadC,
            bind::SDL_KeyCode_SDLK_KP_D => KeypadD,
            bind::SDL_KeyCode_SDLK_KP_E => KeypadE,
            bind::SDL_KeyCode_SDLK_KP_F => KeypadF,
            bind::SDL_KeyCode_SDLK_KP_XOR => KeypadXor,
            bind::SDL_KeyCode_SDLK_KP_POWER => KeypadPower,
            bind::SDL_KeyCode_SDLK_KP_PERCENT => KeypadPercent,
            bind::SDL_KeyCode_SDLK_KP_LESS => KeypadLess,
            bind::SDL_KeyCode_SDLK_KP_GREATER => KeypadGreater,
            bind::SDL_KeyCode_SDLK_KP_AMPERSAND => KeypadAmpersand,
            bind::SDL_KeyCode_SDLK_KP_DBLAMPERSAND => KeypadDblAmpersand,
            bind::SDL_KeyCode_SDLK_KP_VERTICALBAR => KeypadVerticalBar,
            bind::SDL_KeyCode_SDLK_KP_DBLVERTICALBAR => KeypadDblverticalBar,
            bind::SDL_KeyCode_SDLK_KP_COLON => KeypadColon,
            bind::SDL_KeyCode_SDLK_KP_HASH => KeypadHash,
            bind::SDL_KeyCode_SDLK_KP_SPACE => KeypadSpace,
            bind::SDL_KeyCode_SDLK_KP_AT => KeypadAt,
            bind::SDL_KeyCode_SDLK_KP_EXCLAM => KeypadExclam,
            bind::SDL_KeyCode_SDLK_KP_MEMSTORE => KeypadMemStore,
            bind::SDL_KeyCode_SDLK_KP_MEMRECALL => KeypadMemRecall,
            bind::SDL_KeyCode_SDLK_KP_MEMCLEAR => KeypadMemClear,
            bind::SDL_KeyCode_SDLK_KP_MEMADD => KeypadMemAdd,
            bind::SDL_KeyCode_SDLK_KP_MEMSUBTRACT => KeypadMemSubtract,
            bind::SDL_KeyCode_SDLK_KP_MEMMULTIPLY => KeypadMemMultiply,
            bind::SDL_KeyCode_SDLK_KP_MEMDIVIDE => KeypadMemDivide,
            bind::SDL_KeyCode_SDLK_KP_PLUSMINUS => KeypadPlusMinus,
            bind::SDL_KeyCode_SDLK_KP_CLEAR => KeypadClear,
            bind::SDL_KeyCode_SDLK_KP_CLEARENTRY => KeypadClearEntry,
            bind::SDL_KeyCode_SDLK_KP_BINARY => KeypadBinary,
            bind::SDL_KeyCode_SDLK_KP_OCTAL => KeypadOctal,
            bind::SDL_KeyCode_SDLK_KP_DECIMAL => KeypadDecimal,
            bind::SDL_KeyCode_SDLK_KP_HEXADECIMAL => KeypadHexadecimal,
            bind::SDL_KeyCode_SDLK_LCTRL => LCtrl,
            bind::SDL_KeyCode_SDLK_LSHIFT => LShift,
            bind::SDL_KeyCode_SDLK_LALT => LAlt,
            bind::SDL_KeyCode_SDLK_LGUI => LGui,
            bind::SDL_KeyCode_SDLK_RCTRL => RCtrl,
            bind::SDL_KeyCode_SDLK_RSHIFT => RShift,
            bind::SDL_KeyCode_SDLK_RALT => RAlt,
            bind::SDL_KeyCode_SDLK_RGUI => RGui,
            bind::SDL_KeyCode_SDLK_MODE => Mode,
            bind::SDL_KeyCode_SDLK_AUDIONEXT => AudioNext,
            bind::SDL_KeyCode_SDLK_AUDIOPREV => AudioPrev,
            bind::SDL_KeyCode_SDLK_AUDIOSTOP => AudioStop,
            bind::SDL_KeyCode_SDLK_AUDIOPLAY => AudioPlay,
            bind::SDL_KeyCode_SDLK_AUDIOMUTE => AudioMute,
            bind::SDL_KeyCode_SDLK_MEDIASELECT => MediaSelect,
            bind::SDL_KeyCode_SDLK_WWW => Www,
            bind::SDL_KeyCode_SDLK_MAIL => Mail,
            bind::SDL_KeyCode_SDLK_CALCULATOR => Calculator,
            bind::SDL_KeyCode_SDLK_COMPUTER => Computer,
            bind::SDL_KeyCode_SDLK_AC_SEARCH => ApplicationSearch,
            bind::SDL_KeyCode_SDLK_AC_HOME => ApplicationHome,
            bind::SDL_KeyCode_SDLK_AC_BACK => ApplicationBack,
            bind::SDL_KeyCode_SDLK_AC_FORWARD => ApplicationForward,
            bind::SDL_KeyCode_SDLK_AC_STOP => ApplicationStop,
            bind::SDL_KeyCode_SDLK_AC_REFRESH => ApplicationRefresh,
            bind::SDL_KeyCode_SDLK_AC_BOOKMARKS => ApplicationBookmarks,
            bind::SDL_KeyCode_SDLK_BRIGHTNESSDOWN => BrightnessDown,
            bind::SDL_KeyCode_SDLK_BRIGHTNESSUP => BrightnessUp,
            bind::SDL_KeyCode_SDLK_DISPLAYSWITCH => DisplaySwitch,
            bind::SDL_KeyCode_SDLK_KBDILLUMTOGGLE => KeyboardIllumToggle,
            bind::SDL_KeyCode_SDLK_KBDILLUMDOWN => KeyboardIllumDown,
            bind::SDL_KeyCode_SDLK_KBDILLUMUP => KeyboardIllumUp,
            bind::SDL_KeyCode_SDLK_EJECT => Eject,
            bind::SDL_KeyCode_SDLK_SLEEP => Sleep,
            bind::SDL_KeyCode_SDLK_APP1 => App1,
            bind::SDL_KeyCode_SDLK_APP2 => App2,
            bind::SDL_KeyCode_SDLK_AUDIOREWIND => AudioRewind,
            bind::SDL_KeyCode_SDLK_AUDIOFASTFORWARD => AudioFastForward,
            _ => Unknown,
        }
    }
}
