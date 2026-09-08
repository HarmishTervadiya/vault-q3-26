pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6Dm3EYsvKi52MqWw8qRB3upUqHHj6w73wAvGhYiYvpAR");

#[program]
pub mod week2_vault {
    use super::*;

}
