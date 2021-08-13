use crate::bind;

pub fn ram_size() -> usize {
    unsafe { bind::SDL_GetSystemRAM() as usize }
}

// TODO: SIMD allocator

pub mod cpu {
    use crate::bind;

    pub fn count() -> u32 {
        unsafe { bind::SDL_GetCPUCount() as u32 }
    }

    pub fn cache_line_size() -> usize {
        unsafe { bind::SDL_GetCPUCacheLineSize() as usize }
    }

    pub fn simd_alignment() -> usize {
        unsafe { bind::SDL_SIMDGetAlignment() as usize }
    }

    pub fn has_rdtsc() -> bool {
        unsafe { bind::SDL_HasRDTSC() == bind::SDL_bool_SDL_TRUE }
    }
    pub fn has_alti_vec() -> bool {
        unsafe { bind::SDL_HasAltiVec() == bind::SDL_bool_SDL_TRUE }
    }
    pub fn has_mmx() -> bool {
        unsafe { bind::SDL_HasMMX() == bind::SDL_bool_SDL_TRUE }
    }
    pub fn has_3d_now() -> bool {
        unsafe { bind::SDL_Has3DNow() == bind::SDL_bool_SDL_TRUE }
    }
    pub fn has_sse() -> bool {
        unsafe { bind::SDL_HasSSE() == bind::SDL_bool_SDL_TRUE }
    }
    pub fn has_sse2() -> bool {
        unsafe { bind::SDL_HasSSE2() == bind::SDL_bool_SDL_TRUE }
    }
    pub fn has_sse3() -> bool {
        unsafe { bind::SDL_HasSSE3() == bind::SDL_bool_SDL_TRUE }
    }
    pub fn has_sse41() -> bool {
        unsafe { bind::SDL_HasSSE41() == bind::SDL_bool_SDL_TRUE }
    }
    pub fn has_sse42() -> bool {
        unsafe { bind::SDL_HasSSE42() == bind::SDL_bool_SDL_TRUE }
    }
    pub fn has_avx() -> bool {
        unsafe { bind::SDL_HasAVX() == bind::SDL_bool_SDL_TRUE }
    }
    pub fn has_avx2() -> bool {
        unsafe { bind::SDL_HasAVX2() == bind::SDL_bool_SDL_TRUE }
    }
    pub fn has_avx512f() -> bool {
        unsafe { bind::SDL_HasAVX512F() == bind::SDL_bool_SDL_TRUE }
    }
    pub fn has_arm_simd() -> bool {
        unsafe { bind::SDL_HasARMSIMD() == bind::SDL_bool_SDL_TRUE }
    }
    pub fn has_neon() -> bool {
        unsafe { bind::SDL_HasNEON() == bind::SDL_bool_SDL_TRUE }
    }
}
