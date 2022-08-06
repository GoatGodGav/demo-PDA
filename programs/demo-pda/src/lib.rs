use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod demo_pda {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64 ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;

        escrow.from = ctx.accounts.from.key();

        escrow.to = ctx.accounts.to.key();

        escrow.amount = amount;

        Ok(())
    }
}

// create escrow context
#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    //escrow pda
    #[account(
        init,
        seeds = [b"Escrow".as_ref(),from.key().as_ref(), to.key().as_ref()],
        bump,
        payer = from,
        space = size_of::<EscrowAccount>()
    )]
    pub escrow: Account<'info, EscrowAccount>,

    #[account(mut)]
    pub from: Signer<'info>,
    ///CHECK: safe
    #[account(mut)]
    pub to: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

}

// Escrow acount structure
#[account]
pub struct EscrowAccount {
    pub from: Pubkey,

    pub to: Pubkey,

    pub amount: u64
}