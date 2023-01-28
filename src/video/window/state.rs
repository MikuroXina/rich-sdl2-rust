use super::builder::WindowFlags;

/// A format of a [`super::Window`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum WindowFormat {
    /// A normal window.
    Normal,
    /// A full screen window.
    FullScreen,
    /// A full screen window on the current desktop.
    FullScreenWithCurrentDesktop,
    /// A minimized window.
    Minimized,
    /// A maximized window.
    Maximized,
}

impl From<WindowFlags> for WindowFormat {
    fn from(flags: WindowFlags) -> Self {
        if flags.contains(WindowFlags::FULLSCREEN) {
            WindowFormat::FullScreen
        } else if flags.contains(WindowFlags::FULLSCREEN_DESKTOP) {
            WindowFormat::FullScreenWithCurrentDesktop
        } else if flags.contains(WindowFlags::MINIMIZED) {
            WindowFormat::Minimized
        } else if flags.contains(WindowFlags::MAXIMIZED) {
            WindowFormat::Maximized
        } else {
            WindowFormat::Normal
        }
    }
}

/// A kind of a window's context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum WindowContextKind {
    /// Using software render context.
    Software,
    /// Using OpenGL render context.
    OpenGl,
    /// Using Vulkan render context.
    Vulkan,
    /// Using Metal render context.
    Metal,
}

impl From<WindowFlags> for WindowContextKind {
    fn from(flags: WindowFlags) -> Self {
        if flags.contains(WindowFlags::OPENGL) {
            WindowContextKind::OpenGl
        } else if flags.contains(WindowFlags::VULKAN) {
            WindowContextKind::Vulkan
        } else if flags.contains(WindowFlags::METAL) {
            WindowContextKind::Metal
        } else {
            WindowContextKind::Software
        }
    }
}

/// A state of a [`super::Window`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowState {
    /// A format of the window.
    pub format: WindowFormat,
    /// A kind of render context of the window.
    pub context_kind: WindowContextKind,
    /// Whether the window is hidden.
    pub hidden: bool,
    /// Whether the window is borderless.
    pub borderless: bool,
    /// Whether the window is resizable.
    pub resizable: bool,
    /// Whether the window is grabbed the input.
    pub input_grabbed: bool,
    /// Whether the window is on focus.
    pub on_focus: bool,
    /// Whether the window is on the mouse.
    pub on_mouse: bool,
    /// Whether the window comes from the foreign.
    pub foreign: bool,
    /// Whether the window allows high dpi.
    pub allow_high_dpi: bool,
    /// Whether the window captures the mouse.
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
