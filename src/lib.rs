#![no_std]

extern crate alloc;

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::marker::PhantomData;
pub use error::ParserError;

mod ast;
mod disassembler;
mod error;

/// An instance of the assembler/disassembler
pub struct Mipsasm<'a> {
    base_addr: u32,
    #[cfg(not(feature = "std"))]
    _marker: PhantomData<&'a str>,
    debug: bool,
}

impl<'a> Default for Mipsasm<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Mipsasm<'a> {
    /// Create a new `Mipsasm` instance.
    ///
    /// Sets the base address to 0 and the debug flag to false.
    pub fn new() -> Mipsasm<'a> {
        Mipsasm {
            base_addr: 0,
            #[cfg(feature = "std")]
            syms: HashMap::new(),
            #[cfg(not(feature = "std"))]
            _marker: PhantomData,
            debug: false,
        }
    }

    /// Set the base address for the assembler.
    ///
    /// # Examples
    ///
    /// ```
    /// use mipsasm::Mipsasm;
    ///
    /// let mut mipsasm = Mipsasm::new();
    /// mipsasm.base(0x8000_0000);
    /// ```
    pub fn base(&mut self, addr: u32) -> &mut Mipsasm<'a> {
        self.base_addr = addr;
        self
    }

    /// Set the debug flag for the assembler.
    ///
    /// When debug is set to true, the disassembler will print instructions with all extra whitespace stripped.
    /// This is used for testing and will most likely not be useful for other purposes.
    ///
    /// # Examples
    ///
    /// ```
    /// use mipsasm::Mipsasm;
    ///
    /// let mut mipsasm = Mipsasm::new();
    /// mipsasm.debug();
    /// ```
    pub fn debug(&mut self) -> &mut Mipsasm<'a> {
        self.debug = true;
        self
    }

    /// Disassembles a set of MIPS instructions.
    ///
    /// # Examples
    ///
    /// ```
    /// use mipsasm::Mipsasm;
    ///
    /// let mut mipsasm = Mipsasm::new();
    /// let instructions = mipsasm.disassemble(&[0x00850018]);
    /// assert_eq!(instructions, vec!["mult       $a0, $a1"]);
    /// ```
    pub fn disassemble(&self, input: &[u32]) -> Vec<String> {
        let x = disassembler::disassemble(input.to_vec());
        if self.debug {
            x.iter()
                .map(|x| format!("{:?}", x))
                .collect::<Vec<String>>()
        } else {
            x.iter().map(|x| x.to_string()).collect::<Vec<String>>()
        }
    }
}
