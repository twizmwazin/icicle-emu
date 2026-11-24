use crate::ShadowStackState;

pub struct Config {
    pub triple: target_lexicon::Triple,
    /// Overrides the SLEIGH language ID (e.g. `ARM:LE:32:Cortex`) that would otherwise be selected
    /// based on `triple`. The ID is looked up in the `.ldefs` file for the triple's architecture.
    pub language_id: Option<String>,
    pub enable_jit: bool,
    pub enable_jit_mem: bool,
    pub shadow_stack: ShadowStackState,
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
}

impl Default for Config {
    fn default() -> Self {
        Self {
            triple: target_lexicon::Triple::unknown(),
            language_id: None,
            enable_jit: true,
            enable_jit_mem: true,
            shadow_stack: ShadowStackState::Disabled,
            enable_recompilation: true,
            track_uninitialized: false,
            optimize_instructions: true,
            optimize_block: true,
        }
    }
}
