//! Format checker extension for [`RwOps`].

use crate::{bind, file::RwOps};

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ImgFormat {
    Bmp,
    Cur,
    Gif,
    Ico,
    Jpeg,
    Lbm,
    Pcx,
    Png,
    Pnm,
    Tiff,
    Xcf,
    Xpm,
    Xv,
    WebP,
}

/// An extension for checking format of an image.
pub trait ImgChecker {
    /// Determines the format of the image file, or `None` if not supported.
    fn format(&self) -> Option<ImgFormat>;
}

impl ImgChecker for RwOps<'_> {
    fn format(&self) -> Option<ImgFormat> {
        use ImgFormat::*;
        Some(
            if unsafe { bind::IMG_isBMP(self.ptr().as_ptr().cast()) } == 1 {
                Bmp
            } else if unsafe { bind::IMG_isCUR(self.ptr().as_ptr().cast()) } == 1 {
                Cur
            } else if unsafe { bind::IMG_isGIF(self.ptr().as_ptr().cast()) } == 1 {
                Gif
            } else if unsafe { bind::IMG_isICO(self.ptr().as_ptr().cast()) } == 1 {
                Ico
            } else if unsafe { bind::IMG_isJPG(self.ptr().as_ptr().cast()) } == 1 {
                Jpeg
            } else if unsafe { bind::IMG_isLBM(self.ptr().as_ptr().cast()) } == 1 {
                Lbm
            } else if unsafe { bind::IMG_isPCX(self.ptr().as_ptr().cast()) } == 1 {
                Pcx
            } else if unsafe { bind::IMG_isPNG(self.ptr().as_ptr().cast()) } == 1 {
                Png
            } else if unsafe { bind::IMG_isPNM(self.ptr().as_ptr().cast()) } == 1 {
                Pnm
            } else if unsafe { bind::IMG_isTIF(self.ptr().as_ptr().cast()) } == 1 {
                Tiff
            } else if unsafe { bind::IMG_isXCF(self.ptr().as_ptr().cast()) } == 1 {
                Xcf
            } else if unsafe { bind::IMG_isXPM(self.ptr().as_ptr().cast()) } == 1 {
                Xpm
            } else if unsafe { bind::IMG_isXV(self.ptr().as_ptr().cast()) } == 1 {
                Xv
            } else if unsafe { bind::IMG_isWEBP(self.ptr().as_ptr().cast()) } == 1 {
                WebP
            } else {
                return None;
            },
        )
    }
}
