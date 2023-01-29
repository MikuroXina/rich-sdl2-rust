//! Querying some system information.

use crate::bind;

/// Returns the size of the system RAM.
#[must_use]
pub fn ram_size() -> usize {
    unsafe { bind::SDL_GetSystemRAM() as usize }
}

pub mod simd_alloc;

/// System CPU information.
pub mod cpu {
    use crate::bind;

    /// Counts the system CPU.
    #[must_use]
    pub fn count() -> u32 {
        unsafe { bind::SDL_GetCPUCount() as u32 }
    }

    /// Returns the cache line size of the system CPU.
    #[must_use]
    pub fn cache_line_size() -> usize {
        unsafe { bind::SDL_GetCPUCacheLineSize() as usize }
    }

    /// Returns the SIMD alignment of the system CPU.
    #[must_use]
    #[allow(clippy::unnecessary_cast)]
    pub fn simd_alignment() -> usize {
        unsafe { bind::SDL_SIMDGetAlignment() as usize }
    }

    /// Returns whether the system CPU has RDTSC.
    #[must_use]
    pub fn has_rdtsc() -> bool {
        unsafe { bind::SDL_HasRDTSC() == bind::SDL_TRUE }
    }
    /// Returns whether the system CPU has AltiVec.
    #[must_use]
    pub fn has_alti_vec() -> bool {
        unsafe { bind::SDL_HasAltiVec() == bind::SDL_TRUE }
    }
    /// Returns whether the system CPU has MMX.
    #[must_use]
    pub fn has_mmx() -> bool {
        unsafe { bind::SDL_HasMMX() == bind::SDL_TRUE }
    }
    /// Returns whether the system CPU has 3DNow!.
    #[must_use]
    pub fn has_3d_now() -> bool {
        unsafe { bind::SDL_Has3DNow() == bind::SDL_TRUE }
    }
    /// Returns whether the system CPU has SSE.
    #[must_use]
    pub fn has_sse() -> bool {
        unsafe { bind::SDL_HasSSE() == bind::SDL_TRUE }
    }
    /// Returns whether the system CPU has SSE2.
    #[must_use]
    pub fn has_sse2() -> bool {
        unsafe { bind::SDL_HasSSE2() == bind::SDL_TRUE }
    }
    /// Returns whether the system CPU has SSE3.
    #[must_use]
    pub fn has_sse3() -> bool {
        unsafe { bind::SDL_HasSSE3() == bind::SDL_TRUE }
    }
    /// Returns whether the system CPU has SSE4.1.
    #[must_use]
    pub fn has_sse41() -> bool {
        unsafe { bind::SDL_HasSSE41() == bind::SDL_TRUE }
    }
    /// Returns whether the system CPU has SSE4.2.
    #[must_use]
    pub fn has_sse42() -> bool {
        unsafe { bind::SDL_HasSSE42() == bind::SDL_TRUE }
    }
    /// Returns whether the system CPU has AVX.
    #[must_use]
    pub fn has_avx() -> bool {
        unsafe { bind::SDL_HasAVX() == bind::SDL_TRUE }
    }
    /// Returns whether the system CPU has AVX2.
    #[must_use]
    pub fn has_avx2() -> bool {
        unsafe { bind::SDL_HasAVX2() == bind::SDL_TRUE }
    }
    /// Returns whether the system CPU has AVX-512F.
    #[must_use]
    pub fn has_avx512f() -> bool {
        unsafe { bind::SDL_HasAVX512F() == bind::SDL_TRUE }
    }
    /// Returns whether the system CPU has ARM SIMD.
    #[must_use]
    pub fn has_arm_simd() -> bool {
        unsafe { bind::SDL_HasARMSIMD() == bind::SDL_TRUE }
    }
    /// Returns whether the system CPU has NEON, Advanced SIMD.
    #[must_use]
    pub fn has_neon() -> bool {
        unsafe { bind::SDL_HasNEON() == bind::SDL_TRUE }
    }
}
