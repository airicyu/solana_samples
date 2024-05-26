use anchor_lang::prelude::*;

declare_id!("14sUS6ibSoXE6bLrFZGv1md1fRdpDu3Z2T4zfpB1UpU3");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize_counter(ctx: Context<InitializeCounterContext>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0u64;
        Ok(())
    }

    pub fn increase_counter(ctx: Context<IncreaseCounterContext>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1u64;

        msg!("Counter increased to {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounterContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, space = 8 + 8, seeds = [b"counter".as_ref()], bump)]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub count: u64,
}

#[derive(Accounts)]
pub struct IncreaseCounterContext<'info> {
    #[account(mut, seeds = [b"counter".as_ref()], bump)]
    pub counter: Account<'info, Counter>,
}
