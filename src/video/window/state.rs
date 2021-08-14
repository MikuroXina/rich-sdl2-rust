use super::WindowFlags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowFormat {
    Normal,
    FullScreen,
    FullScreenWithCurrentDesktop,
    Minimized,
    Maximized,
}

impl From<WindowFlags> for WindowFormat {
    fn from(flags: WindowFlags) -> Self {
        use WindowFormat::*;
        if flags.contains(WindowFlags::FULLSCREEN) {
            FullScreen
        } else if flags.contains(WindowFlags::FULLSCREEN_DESKTOP) {
            FullScreenWithCurrentDesktop
        } else if flags.contains(WindowFlags::MINIMIZED) {
            Minimized
        } else if flags.contains(WindowFlags::MAXIMIZED) {
            Maximized
        } else {
            Normal
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowContextKind {
    Software,
    OpenGl,
    Vulkan,
    Metal,
}

impl From<WindowFlags> for WindowContextKind {
    fn from(flags: WindowFlags) -> Self {
        use WindowContextKind::*;
        if flags.contains(WindowFlags::OPENGL) {
            OpenGl
        } else if flags.contains(WindowFlags::VULKAN) {
            Vulkan
        } else if flags.contains(WindowFlags::METAL) {
            Metal
        } else {
            Software
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowState {
    pub format: WindowFormat,
    pub context_kind: WindowContextKind,
    pub hidden: bool,
    pub borderless: bool,
    pub resizable: bool,
    pub input_grabbed: bool,
    pub on_focus: bool,
    pub on_mouse: bool,
    pub foreign: bool,
    pub allow_high_dpi: bool,
    pub mouse_capture: bool,
}

impl From<WindowFlags> for WindowState {
    fn from(flags: WindowFlags) -> Self {
        Self {
            format: flags.into(),
            context_kind: flags.into(),
            hidden: flags.contains(WindowFlags::HIDDEN),
            borderless: flags.contains(WindowFlags::BORDERLESS),
            resizable: flags.contains(WindowFlags::RESIZABLE),
            input_grabbed: flags.contains(WindowFlags::INPUT_GRABBED),
            on_focus: flags.contains(WindowFlags::INPUT_FOCUS),
            on_mouse: flags.contains(WindowFlags::MOUSE_FOCUS),
            foreign: flags.contains(WindowFlags::FOREIGN),
            allow_high_dpi: flags.contains(WindowFlags::ALLOW_HIGHDPI),
            mouse_capture: flags.contains(WindowFlags::MOUSE_CAPTURE),
        }
    }
}
