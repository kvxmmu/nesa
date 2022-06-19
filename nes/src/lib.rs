#![feature(generic_arg_infer)]

pub mod cpu;
pub mod cpu_status;

pub mod cpu_registers;

pub mod decoder;

pub mod opcode;
pub mod memory;

#[cfg(test)]
mod tests;
