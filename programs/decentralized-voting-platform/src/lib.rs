pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6FuguLE3hjvTaexSGjN2GtGPX9jxddvmBXWj5ddBBWqp");

#[program]
pub mod decentralized_voting_platform {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
