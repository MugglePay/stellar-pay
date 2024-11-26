#![no_std]

mod contract;
mod error;
mod types;
mod events;
mod admin;
mod slippage;

#[cfg(test)]
mod test;

pub use contract::MuggleDex;
