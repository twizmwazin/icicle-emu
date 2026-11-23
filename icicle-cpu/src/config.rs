use crate::ShadowStackState;

pub struct Config {
    /// The id of the SLEIGH language (e.g. `ARM:LE:32:v8`) to use for the emulator.
    pub language_id: String,
    /// The target triple of the emulator, used to configure architecture and operating system
    /// specific behaviour. If the architecture is unknown, it is inferred from the language.
    pub triple: target_lexicon::Triple,
    pub enable_jit: bool,
    pub enable_jit_mem: bool,
    pub shadow_stack: ShadowStackState,
    pub enable_recompilation: bool,
    pub track_uninitialized: bool,
    pub optimize_instructions: bool,
    pub optimize_block: bool,
}

impl Config {
    pub fn from_language_id(language_id: &str) -> Self {
        Self { language_id: language_id.into(), ..Default::default() }
    }

    pub fn from_target_triple(triple: &str) -> Self {
        Self::from_triple(triple.parse().unwrap_or_else(|_| target_lexicon::Triple::unknown()))
    }

    /// Create a config for `triple` using the default SLEIGH language for the target. The language
    /// is left unset if the target is not supported.
    pub fn from_triple(triple: target_lexicon::Triple) -> Self {
        let language_id = language_id_for_triple(&triple).unwrap_or_default().into();
        Self { language_id, triple, ..Default::default() }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language_id: String::new(),
            triple: target_lexicon::Triple::unknown(),
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

/// Get the id of the default SLEIGH language for `triple`, or `None` if the target is unsupported.
pub fn language_id_for_triple(triple: &target_lexicon::Triple) -> Option<&'static str> {
    use target_lexicon::{
        Aarch64Architecture, Architecture, ArmArchitecture, Mips32Architecture,
        Riscv32Architecture, Riscv64Architecture,
    };

    Some(match triple.architecture {
        Architecture::Arm(variant) => match variant {
            ArmArchitecture::Arm => "ARM:LE:32:v8",
            ArmArchitecture::Armeb => "ARM:BE:32:v8",

            ArmArchitecture::Armv4 => "ARM:LE:32:v4",
            ArmArchitecture::Armv4t => "ARM:LE:32:v4t",
            ArmArchitecture::Armv5t | ArmArchitecture::Armv5te | ArmArchitecture::Armv5tej => {
                "ARM:LE:32:v5t"
            }
            ArmArchitecture::Armv6
            | ArmArchitecture::Armv6j
            | ArmArchitecture::Armv6k
            | ArmArchitecture::Armv6z
            | ArmArchitecture::Armv6kz
            | ArmArchitecture::Armv6t2
            | ArmArchitecture::Armv6m => "ARM:LE:32:v6",

            ArmArchitecture::Armv7
            | ArmArchitecture::Armv7a
            | ArmArchitecture::Armv7k
            | ArmArchitecture::Armv7ve
            | ArmArchitecture::Armv7m
            | ArmArchitecture::Armv7r
            | ArmArchitecture::Armv7s => "ARM:LE:32:v7",

            ArmArchitecture::Armebv7r => "ARM:BE:32:v7",

            ArmArchitecture::Armv8
            | ArmArchitecture::Armv8a
            | ArmArchitecture::Armv8_1a
            | ArmArchitecture::Armv8_2a
            | ArmArchitecture::Armv8_3a
            | ArmArchitecture::Armv8_4a
            | ArmArchitecture::Armv8_5a
            | ArmArchitecture::Armv8mBase
            | ArmArchitecture::Armv8mMain
            | ArmArchitecture::Armv8r => "ARM:LE:32:v8",

            ArmArchitecture::Thumbv4t => "ARM:LE:32:v8T",
            ArmArchitecture::Thumbv5te => "ARM:LE:32:v8T",
            ArmArchitecture::Thumbv6m => "ARM:LE:32:v8T",

            // No specific v7 thumb target
            ArmArchitecture::Thumbv7a
            | ArmArchitecture::Thumbv7em
            | ArmArchitecture::Thumbv7m
            | ArmArchitecture::Thumbv7neon => "ARM:LE:32:v8T",

            ArmArchitecture::Thumbv8mBase | ArmArchitecture::Thumbv8mMain => "ARM:LE:32:v8T",

            ArmArchitecture::Thumbeb => "ARM:BE:32:v8T",
            _ => return None,
        },
        Architecture::Aarch64(variant) => match variant {
            Aarch64Architecture::Aarch64 => "AARCH64:LE:64:v8A",
            Aarch64Architecture::Aarch64be => "AARCH64:BE:64:v8A",
            _ => return None,
        },
        Architecture::M68k => "68000:BE:32:Coldfire",
        Architecture::Mips32(variant) => match variant {
            Mips32Architecture::Mips => "MIPS:BE:32:default",
            Mips32Architecture::Mipsel => "MIPS:LE:32:default",
            Mips32Architecture::Mipsisa32r6 => "MIPS:BE:32:R6",
            Mips32Architecture::Mipsisa32r6el => "MIPS:LE:32:R6",
            _ => return None,
        },
        Architecture::Msp430 => "TI_MSP430X:LE:32:default",
        Architecture::Powerpc => "PowerPC:BE:32:default",
        Architecture::Powerpc64 => "PowerPC:BE:64:default",
        Architecture::Powerpc64le => "PowerPC:LE:64:default",
        Architecture::Riscv32(variant) => match variant {
            Riscv32Architecture::Riscv32 => "RISCV:LE:32:default",
            Riscv32Architecture::Riscv32gc => "RISCV:LE:32:RV32GC",
            Riscv32Architecture::Riscv32i => "RISCV:LE:32:RV32I",
            Riscv32Architecture::Riscv32imc => "RISCV:LE:32:RV32IMC",
            _ => return None,
        },
        Architecture::Riscv64(variant) => match variant {
            Riscv64Architecture::Riscv64 => "RISCV:LE:64:default",
            Riscv64Architecture::Riscv64gc => "RISCV:LE:64:RV64GC",
            _ => return None,
        },
        Architecture::X86_32(_) => "x86:LE:32:default",
        Architecture::X86_64h | Architecture::X86_64 => "x86:LE:64:default",
        Architecture::XTensa => "Xtensa:LE:32:default",
        _ => return None,
    })
}
