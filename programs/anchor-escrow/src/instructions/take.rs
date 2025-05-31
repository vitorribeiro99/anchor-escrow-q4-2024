use std::str::FromStr;

use anchor_lang::{prelude::*, solana_program::system_instruction::transfer, system_program::{transfer, Transfer}};

use mpl_core::instructions::TransferV1CpiBuilder;

use anchor_spl::{
    token_interface::{
        close_account, 
        TokenInterface, 
        CloseAccount, 
    },
};

use crate::state::{Escrow, VaultState};

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    /// CHECK: This is verified by the CPI
    #[account(mut)]
    pub mint_b: UncheckedAccount<'info>,
    /// CHECK: This is verified by the CPI
    #[account(mut, address = Pubkey::from_str("").unwrap())]
    pub collection: UncheckedAccount<'info>,
    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    escrow: Account<'info, Escrow>,
    #[account(
        mut,
        seeds = [b"state", maker.key().as_ref()],
        bump = vault_state.state_bump,  
        close = maker 
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,  
    )]
    pub vault: SystemAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the ID of the Metaplex Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> Take<'info> {
    pub fn deposit(&mut self) -> Result<()> {

        let binding = self.mint_b.key();
        let binding2 = self.escrow.key();
        
        let signer_seeds: [&[&[u8]]; 1] = [&[
            binding2.as_ref(),
            binding.as_ref(),
        ]];

        TransferV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.mint_b.to_account_info())
            .authority(Some(&self.taker.to_account_info()))
            .payer(&self.taker.to_account_info())
            .new_owner(&self.escrow.to_account_info()) 
            .collection(Some(&self.collection.to_account_info()))
            .system_program(Some(&self.system_program.to_account_info()))
            .invoke_signed(&signer_seeds)?;
        Ok(())
    }

    pub fn withdraw_and_close_vault(&mut self) -> Result<()> {

        let binding = self.mint_b.key();
        let binding2 = self.escrow.key();
        
        let signer_seeds: [&[&[u8]]; 1] = [&[
            binding2.as_ref(),
            binding.as_ref(),
        ]];

        TransferV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.mint_b.to_account_info())
            .authority(Some(&self.escrow.to_account_info()))
            .payer(&self.taker.to_account_info())
            .new_owner(&self.taker.to_account_info()) 
            .collection(Some(&self.collection.to_account_info()))
            .system_program(Some(&self.system_program.to_account_info()))
            .invoke_signed(&signer_seeds)?;

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.taker.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, self.vault.lamports());

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];

        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.taker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            &signer_seeds,
        );

        close_account(ctx)
    }
}