use std::mem::MaybeUninit;

use super::info::RendererInfo;
use crate::bind;

pub fn drivers() -> Vec<RendererInfo> {
    let num = unsafe {
        bind::SDL_InitSubSystem(bind::SDL_INIT_VIDEO);
        bind::SDL_GetNumRenderDrivers()
    };
    (0..num)
        .map(|index| {
            let mut info = MaybeUninit::<bind::SDL_RendererInfo>::uninit();
            let ret = unsafe { bind::SDL_GetRenderDriverInfo(index, info.as_mut_ptr()) };
            unsafe { info.assume_init() }.into()
        })
        .collect()
}
