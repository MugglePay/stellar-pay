#![no_std]

mod contract;
mod error;
mod types;
mod events;

#[cfg(test)]
mod test;

pub use contract::MuggleDex;
