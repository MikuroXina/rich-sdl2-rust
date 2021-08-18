use bitflags::bitflags;

bitflags! {
    /// A flag to control OpenGL context.
    pub struct GlContextFlag: u32 {
        /// This flag is currently ignored on other targets that don't support equivalent functionality. This flag is intended to put OpenGL into a "debug" mode which might offer better developer insights, possibly at a loss of performance (although a given OpenGL implementation may or may not do anything differently in the presence of this flag).
        const DEBUG = 1 << 0;
        /// This flag is currently ignored on other targets that don't support equivalent functionality. This flag is intended to put OpenGL into a "forward compatible" mode, which means that no deprecated functionality will be supported, possibly at a gain in performance, and only applies to OpenGL 3.0 and later contexts.
        const FORWARD_COMPATIBLE = 1 << 1;
        /// This flag is currently ignored on other targets that don't support equivalent functionality. This flag is intended to require an OpenGL context that supports the `GL_ARB_robustness` extension -- a mode that offers a few APIs that are safer than the usual defaults.
        const ROBUST_ACCESS = 1 << 2;
        /// This flag is currently ignored on other targets that don't support equivalent functionality. This flag is intended to require OpenGL to make promises about what to do in the face of driver or hardware failure.
        const RESET_ISOLATION = 1 << 3;
    }
}
