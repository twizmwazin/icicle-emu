pub struct Config {
    pub triple: target_lexicon::Triple,
    /// The SLEIGH language id (e.g. `ARM:LE:32:v8`) to use for the emulator. If set, this is used
    /// to select the SLEIGH specification instead of the target triple. If the `triple` is unknown,
    /// it is inferred from the language.
    pub language_id: Option<String>,
    pub enable_jit: bool,
    pub enable_jit_mem: bool,
    pub enable_shadow_stack: bool,
    pub enable_recompilation: bool,
    pub track_uninitialized: bool,
    pub optimize_instructions: bool,
    pub optimize_block: bool,
}

impl Config {
    pub fn from_target_triple(triple: &str) -> Self {
        Self {
            triple: triple.parse().unwrap_or_else(|_| target_lexicon::Triple::unknown()),
            ..Default::default()
        }
    }

    pub fn from_language_id(language_id: &str) -> Self {
        Self { language_id: Some(language_id.into()), ..Default::default() }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            triple: target_lexicon::Triple::unknown(),
            language_id: None,
            enable_jit: true,
            enable_jit_mem: true,
            enable_shadow_stack: true,
            enable_recompilation: true,
            track_uninitialized: false,
            optimize_instructions: true,
            optimize_block: true,
        }
    }
}
