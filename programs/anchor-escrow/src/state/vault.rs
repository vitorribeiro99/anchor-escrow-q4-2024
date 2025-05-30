use anchor_lang::prelude::*;

#[account]
pub struct VaultState {
    pub state_bump: u8,
    pub vault_bump: u8,
}

impl VaultState {
    pub const LEN: usize = 8 + 1 + 1; // 8 (discriminator) + 1 (state_bump) + 1 (vault_bump)
}