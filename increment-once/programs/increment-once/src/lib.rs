use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// ===== Goal =====
// Each account can only increment the counter ONCE.
//
// ===== Implementation =====
// We want one PDA per user account.
// Each PDA will store whether that user has incremented the counter yet.

#[program]
mod increment_once {
    use super::*;

    pub fn create(ctx: Context<Create>, bump: u8) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.bump = bump;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>, _bump: u8) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;

        // Don't actually need to check the value, because if one user tries to increment
        // twice, it will fail because the has_incremented PDA is already in use.
        let has_incremented = &mut ctx.accounts.has_incremented;
        has_incremented.has_incremented = true;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Create<'info> {
    #[account(init, seeds = [b"counter".as_ref()], bump = bump, payer = user)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_bump: u8)]
pub struct Increment<'info> {
    #[account(mut, seeds = [b"counter".as_ref()], bump = counter.bump)]
    pub counter: Account<'info, Counter>,
    #[account(init, seeds = [user.key().as_ref()], bump = _bump, payer = user)]
    pub has_incremented: Account<'info, HasIncremented>,
    // Why do we need mut? It's kind of sneaky—when the user is the wallet (provider.wallet.publicKey)
    // this is automatically mutable. But if you use another public key, it needs to be mutable,
    // since paying for stuff (e.g. paying for the tx) counts as modifying an account.
    // Note, receiving money also counts as a mutation!
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Holds counter state.
#[account]
#[derive(Default)]
pub struct Counter {
    pub count: u64,
    pub bump: u8,
}

// Should be one of these per user that wants to increment.
#[account]
#[derive(Default)]
pub struct HasIncremented {
    // Technically not necessary... see increment function.
    pub has_incremented: bool,
}
