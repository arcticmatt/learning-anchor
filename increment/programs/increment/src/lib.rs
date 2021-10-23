use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

//
// A simple program that lets any user increment a counter as many times as they want.
//

#[program]
mod increment {
    use super::*;

    pub fn create(ctx: Context<Create>, bump: u8) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.bump = bump;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Create<'info> {
    // Program derived address
    #[account(init, seeds = [b"counter".as_ref()], bump = bump, payer = user)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [b"counter".as_ref()], bump = counter.bump)]
    pub counter: Account<'info, Counter>,
}

// Holds counter state.
#[account]
#[derive(Default)]
pub struct Counter {
    pub count: u64,
    pub bump: u8,
}
