//! Extensions for [`crate::renderer::pen::Pen`].

use super::{RenderExt, RenderMode};
use crate::{
    color::Rgba,
    geo::{Point, Rect, Size},
    renderer::{pen::Pen, Paster},
    texture::Texture,
    ttf::font::Font,
};

/// X-axis alignment of the text.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TextAlignX {
    /// Aligns to the left. Moves the text to the right so that the pivot comes the left edge.
    Left,
    /// Aligns to the center.
    Center,
    /// Aligns to the right. Moves the text to the left so that the pivot comes the right edge.
    Right,
}

impl Default for TextAlignX {
    fn default() -> Self {
        Self::Left
    }
}

/// Y-axis alignment of the text.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TextAlignY {
    /// Aligns to the top. Moves the text to the down so that the pivot comes the top edge.
    Top,
    /// Aligns to the center.
    Center,
    /// Aligns to the bottom. Moves the text to the up so that the pivot comes the bottom edge.
    Bottom,
}

impl Default for TextAlignY {
    fn default() -> Self {
        Self::Top
    }
}

/// Alignments of the text.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TextAlign {
    /// The X-axis alignment of the text.
    pub x: TextAlignX,
    /// The Y-axis alignment of the text.
    pub y: TextAlignY,
}

/// Options to render the text for [`FontRenderExt::text`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontRenderOptions {
    align: TextAlign,
    pivot: Point,
    mode: RenderMode,
}

impl FontRenderOptions {
    /// Constructs a default option.
    pub fn new() -> Self {
        FontRenderOptions {
            align: TextAlign::default(),
            pivot: Point::default(),
            mode: RenderMode::Blended {
                foreground: Rgba {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255,
                },
            },
        }
    }

    /// Sets the render mode with color.
    pub fn mode(mut self, mode: RenderMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the alignment.
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets the pivot point.
    pub fn pivot(mut self, pivot: Point) -> Self {
        self.pivot = pivot;
        self
    }

    fn aligned_pos(&self, Size { width, height }: Size) -> Point {
        let x = match self.align.x {
            TextAlignX::Left => self.pivot.x,
            TextAlignX::Center => self.pivot.x - width as i32 / 2,
            TextAlignX::Right => self.pivot.x - width as i32,
        };
        let y = match self.align.y {
            TextAlignY::Top => self.pivot.y,
            TextAlignY::Center => self.pivot.y - height as i32 / 2,
            TextAlignY::Bottom => self.pivot.y - height as i32,
        };
        Point { x, y }
    }
}

impl Default for FontRenderOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// An extension for [`Pen`] to render a text.
pub trait FontRenderExt {
    /// Renders a text to the area with the font.
    fn text(&self, font: &Font, text: &str, options: FontRenderOptions);
}

impl FontRenderExt for Pen<'_> {
    fn text(&self, font: &Font, text: &str, options: FontRenderOptions) {
        if text.is_empty() {
            return;
        }
        let surface = font
            .render(text, options.mode)
            .expect("rendering text failed");
        let texture = Texture::from_surface(self.renderer(), &surface);
        let size = font
            .rendered_size(text)
            .expect("calculating text size failed");
        let up_left = options.aligned_pos(size);
        Paster::new(self.renderer()).paste(&texture, Some(Rect { up_left, size }));
    }
}
