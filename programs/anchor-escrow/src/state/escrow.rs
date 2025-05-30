use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64, // A random number used to generate the escrow account's address.
    pub maker: Pubkey, // The account that created the escrow account.
    // pub mint_a: Pubkey, // The mint of the token the maker is trading with. In our case, will be SOL.
    pub mint_b: Pubkey, // The mint of the token the maker is trading for.
    pub receive: u64, // The amount of tokens that need to be received before the funds are released.
    pub bump: u8, // Since our Escrow account will be a PDA, we will store the bump of the account.
}

