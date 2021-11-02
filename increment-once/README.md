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

Here's what `system_instruction::create_account` looks like ([source](https://github.com/solana-labs/solana/blob/master/sdk/program/src/system_instruction.rs)):

```
pub fn create_account(
    from_pubkey: &Pubkey,
    to_pubkey: &Pubkey,
    lamports: u64,
    space: u64,
    owner: &Pubkey,
) -> Instruction {
    let account_metas = vec![
        AccountMeta::new(*from_pubkey, true),
        AccountMeta::new(*to_pubkey, true),
    ];
    Instruction::new_with_bincode(
        system_program::id(),
        &SystemInstruction::CreateAccount {
            lamports,
            space,
            owner: *owner,
        },
        account_metas,
    )
}
```

Notably, `create_account` is used, not `create_account_with_seed`. The key thing is that `invoke_signed` is used. `invoke_signed` reconstructs the `has_incremented` PDA using the passed-in seeds and the caller's program ID, and compares it to `has_incremented.to_account_info()`. See [here](https://github.com/solana-labs/solana/blob/master/program-test/src/lib.rs#L301-L323) for the code.

```
// Check Signers
// account_infos is the second arg of invoke_signed.
for account_info in account_infos {
    // instruction.accounts contains the first two args of create_account.
    for instruction_account in &instruction.accounts {
        // This will be true when instruction_account is has_incremented.to_account_info().key
        // and account_info is has_incremented.to_account_info().
        if *account_info.unsigned_key() == instruction_account.pubkey
            && instruction_account.is_signer
            && !account_info.is_signer
        {
            let mut program_signer = false;
            for seeds in signers_seeds.iter() {
                let signer = Pubkey::create_program_address(seeds, &caller).unwrap();
                if instruction_account.pubkey == signer {
                    program_signer = true;
                    break;
                }
            }
            assert!(
                program_signer,
                "Missing signer for {}",
                instruction_account.pubkey
            );
        }
    }
}
```

The above code just makes sure that the account being created (`has_incremented.to_account_info().key`) is also a signer of the instruction. Effectively, this means that if a PDA was generated with a specific program ID, only that program can create the account by calling `invoke_signed` using the PDA's seeds.

One important thing to note is that the program that generates a PDA may not necessarily own it. In the code above, `program_id` happens to be passed as the owner, but this is not enforced (I think). Importantly, the program that owns the PDA account is the only program allowed to modify its data.

However, it is impossible for `programA` to generate a PDA using `findPda(programB, seeds)` and assign itself as the owner. Why? Because `programA` is only able to call `invoke_signed` on `create_account` instructions that create PDAs with `programA` as the program.

In other words, `programA` can generate a PDA using `findPda(programA, seeds)` and *grant* ownership to another program. But it cannot *steal* ownership of another program's PDA.

The [Associated Token Account Program](https://spl.solana.com/associated-token-account) is a good example where the program that creates a PDA is different than its owner. See the code below ([source1](https://github.com/solana-labs/solana-program-library/blob/master/associated-token-account/program/src/processor.rs#L77-L85) and [source2](https://github.com/solana-labs/solana-program-library/blob/master/associated-token-account/program/src/tools/account.rs#L51-L65)):


```
create_pda_account(
    funder_info,
    &rent,
    spl_token::state::Account::LEN,
    spl_token_program_id,
    system_program_info,
    associated_token_account_info,
    associated_token_account_signer_seeds,
)?;
```

```
pub fn create_pda_account<'a>(
    payer: &AccountInfo<'a>,
    rent: &Rent,
    space: usize,
    owner: &Pubkey,
    system_program: &AccountInfo<'a>,
    new_pda_account: &AccountInfo<'a>,
    new_pda_signer_seeds: &[&[u8]],
) -> ProgramResult {
  ...
  invoke_signed(
      &system_instruction::create_account(
          payer.key,
          new_pda_account.key,
          rent.minimum_balance(space).max(1),
          space as u64,
          owner,
      ),
      &[
          payer.clone(),
          new_pda_account.clone(),
          system_program.clone(),
      ],
      &[new_pda_signer_seeds],
  )
}
```

Here, the SPL Token Program is the PDA's owner, but the Associated Token Account Program created the PDA. This means the SPL Token Program is allowed to modify the account's state, e.g. increase its balance.