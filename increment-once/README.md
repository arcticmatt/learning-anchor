# increment-once

Similar to increment, but each user can only increment the counter once.

## PDAs

To generate expanded code, run:

```
rustup install nightly
rustup run nightly cargo expand
```

This shows how the `has_incremented` PDA gets created:

```
anchor_lang::solana_program::program::invoke_signed(
    &anchor_lang::solana_program::system_instruction::create_account(
        payer.to_account_info().key,
        // Account to create
        has_incremented.to_account_info().key,
        lamports,
        space as u64,
        // Owner
        program_id,
    ),
    &[
        payer.to_account_info(),
        has_incremented.to_account_info(),
        system_program.to_account_info(),
    ],
    &[&[user.key().as_ref(), &[_bump]][..]],
)?;
```

Notably, `create_account` is used, not `create_account_with_seed`. The key thing is that `invoke_signed` is used. `invoke_signed` reconstructs the `has_incremented` PDA using the passed-in seeds and the caller's program ID, and compares it to `has_incremented.to_account_info()`. See [here](https://github.com/solana-labs/solana/blob/master/program-test/src/lib.rs#L301-L323) for the code.