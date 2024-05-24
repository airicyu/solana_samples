use anchor_lang::prelude::*;

declare_id!("7hVw37hUSM7aZConaSHExcbNyN2kqvNqvWWMWFYyieB7");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize_counter(
        ctx: Context<InitializeCounterContext>,
        data: InitializeCounterInstructionData
    ) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = data.init_count;
        msg!("Counter initialized!");
        Ok(())
    }

    pub fn increase_count(ctx: Context<IncreaseCountContext>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("Counter increased! New count: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(data: InitializeCounterInstructionData)]
pub struct InitializeCounterContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer = payer, seeds = [b"counter"], bump, space = 8 + 8)]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct InitializeCounterInstructionData {
    pub init_count: u64,
}

#[account]
pub struct Counter {
    count: u64,
}

#[derive(Accounts)]
pub struct IncreaseCountContext<'info> {
    #[account(mut, seeds = [b"counter"], bump)]
    pub counter: Account<'info, Counter>,
}
