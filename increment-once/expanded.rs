#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use anchor_lang::prelude::*;
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
        218u8, 7u8, 92u8, 178u8, 255u8, 94u8, 198u8, 129u8, 118u8, 19u8, 222u8, 83u8, 11u8, 105u8,
        42u8, 135u8, 53u8, 71u8, 119u8, 105u8, 218u8, 71u8, 67u8, 12u8, 189u8, 129u8, 84u8, 51u8,
        92u8, 74u8, 131u8, 39u8,
    ]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
use increment_once::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) =
        unsafe { ::solana_program::entrypoint::deserialize(input) };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one of three categories, each of which
/// can be considered a different "namespace" of the program.
///
/// 1) Global methods - regular methods inside of the `#[program]`.
/// 2) State methods - associated methods inside a `#[state]` struct.
/// 3) Interface methods - methods inside a strait struct's
///    implementation of an `#[interface]` trait.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Strip method identifier off the first 8 bytes of the instruction
///   data and invoke the identified method. The method identifier
///   is a variant of sighash. See docs.rs for `anchor_lang` for details.
/// * If the method identifier is an IDL identifier, execute the IDL
///   instructions, which are a special set of hardcoded instructions
///   baked into every Anchor program. Then exit.
/// * Otherwise, the method identifier is for a user defined
///   instruction, i.e., one of the methods in the user defined
///   `#[program]` module. Perform method dispatch, i.e., execute the
///   big match statement mapping method identifier to method handler
///   wrapper.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
#[cfg(not(feature = "no-entrypoint"))]
pub fn entry(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    if data.len() < 8 {
        return Err(anchor_lang::__private::ErrorCode::InstructionMissing.into());
    }
    dispatch(program_id, accounts, data).map_err(|e| {
        ::solana_program::log::sol_log(&e.to_string());
        e
    })
}
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct IncrementOnce;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for IncrementOnce {
        #[inline]
        fn clone(&self) -> IncrementOnce {
            match *self {
                IncrementOnce => IncrementOnce,
            }
        }
    }
    impl anchor_lang::AccountDeserialize for IncrementOnce {
        fn try_deserialize(
            buf: &mut &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            IncrementOnce::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(
            _buf: &mut &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            Ok(IncrementOnce)
        }
    }
    impl anchor_lang::Id for IncrementOnce {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each method in an anchor program is uniquely defined by a namespace
/// and a rust identifier (i.e., the name given to the method). These
/// two pieces can be combined to creater a method identifier,
/// specifically, Anchor uses
///
/// Sha256("<namespace>::<rust-identifier>")[..8],
///
/// where the namespace can be one of three types. 1) "global" for a
/// regular instruction, 2) "state" for a state struct instruction
/// handler and 3) a trait namespace (used in combination with the
/// `#[interface]` attribute), which is defined by the trait name, e..
/// `MyTrait`.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    if true {
        if sighash == anchor_lang::idl::IDL_IX_TAG.to_le_bytes() {
            return __private::__idl::__idl_dispatch(program_id, accounts, &ix_data);
        }
    }
    match sighash {
        [24, 30, 200, 40, 5, 28, 7, 119] => {
            __private::__global::create(program_id, accounts, ix_data)
        }
        [11, 18, 104, 9, 104, 174, 59, 33] => {
            __private::__global::increment(program_id, accounts, ix_data)
        }
        _ => Err(anchor_lang::__private::ErrorCode::InstructionFallbackNotFound.into()),
    }
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            idl_ix_data: &[u8],
        ) -> ProgramResult {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut accounts = anchor_lang::idl::IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut accounts = anchor_lang::idl::IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut accounts = anchor_lang::idl::IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateAccounts,
            data_len: u64,
        ) -> ProgramResult {
            if program_id != accounts.program.key {
                return Err(anchor_lang::__private::ErrorCode::IdlInstructionInvalidProgram.into());
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = anchor_lang::idl::IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = 8 + 32 + 4 + data_len as usize;
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.clone(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                anchor_lang::idl::IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateBuffer,
        ) -> ProgramResult {
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            idl_data: Vec<u8>,
        ) -> ProgramResult {
            let mut idl = &mut accounts.idl;
            idl.data.extend(idl_data);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            new_authority: Pubkey,
        ) -> ProgramResult {
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlSetBuffer,
        ) -> ProgramResult {
            accounts.idl.data = accounts.buffer.data.clone();
            Ok(())
        }
    }
    /// __state mod defines wrapped handlers for state instructions.
    pub mod __state {
        use super::*;
    }
    /// __interface mod defines wrapped handlers for `#[interface]` trait
    /// implementations.
    pub mod __interface {
        use super::*;
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn create(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::Create::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Create { bump } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = Create::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            increment_once::create(
                Context::new(program_id, &mut accounts, remaining_accounts),
                bump,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn increment(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::Increment::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Increment { _bump } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                Increment::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            increment_once::increment(
                Context::new(program_id, &mut accounts, remaining_accounts),
                _bump,
            )?;
            accounts.exit(program_id)
        }
    }
}
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
        let has_incremented = &mut ctx.accounts.has_incremented;
        has_incremented.has_incremented = true;
        Ok(())
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction struct definitions for `#[state]` methods.
    pub mod state {
        use super::*;
    }
    /// Instruction.
    pub struct Create {
        pub bump: u8,
    }
    impl borsh::ser::BorshSerialize for Create
    where
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.bump, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Create
    where
        u8: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                bump: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for Create {
        fn data(&self) -> Vec<u8> {
            let mut d = [24, 30, 200, 40, 5, 28, 7, 119].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Increment {
        pub _bump: u8,
    }
    impl borsh::ser::BorshSerialize for Increment
    where
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._bump, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Increment
    where
        u8: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _bump: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for Increment {
        fn data(&self) -> Vec<u8> {
            let mut d = [11, 18, 104, 9, 104, 174, 59, 33].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_create::*;
    pub use crate::__client_accounts_increment::*;
}
# [instruction (bump : u8)]
pub struct Create<'info> {
    # [account (init , seeds = [b"counter" . as_ref ()] , bump = bump , payer = user)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for Create<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let mut ix_data = ix_data;
        struct __Args {
            bump: u8,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.bump, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for __Args
        where
            u8: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    bump: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        let __Args { bump } = __Args::deserialize(&mut ix_data)
            .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
        let counter = &accounts[0];
        *accounts = &accounts[1..];
        let user: Signer = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let system_program: anchor_lang::Program<System> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let __anchor_rent = Rent::get()?;
        let counter = {
            let space = 8 + Counter::default().try_to_vec().unwrap().len();
            let payer = user.to_account_info();
            let __current_lamports = counter.to_account_info().lamports();
            if __current_lamports == 0 {
                let lamports = __anchor_rent.minimum_balance(space);
                anchor_lang::solana_program::program::invoke_signed(
                    &anchor_lang::solana_program::system_instruction::create_account(
                        payer.to_account_info().key,
                        counter.to_account_info().key,
                        lamports,
                        space as u64,
                        program_id,
                    ),
                    &[
                        payer.to_account_info(),
                        counter.to_account_info(),
                        system_program.to_account_info(),
                    ],
                    &[&[b"counter".as_ref(), &[bump]][..]],
                )?;
            } else {
                let required_lamports = __anchor_rent
                    .minimum_balance(space)
                    .max(1)
                    .saturating_sub(__current_lamports);
                if required_lamports > 0 {
                    anchor_lang::solana_program::program::invoke(
                        &anchor_lang::solana_program::system_instruction::transfer(
                            payer.to_account_info().key,
                            counter.to_account_info().key,
                            required_lamports,
                        ),
                        &[
                            payer.to_account_info(),
                            counter.to_account_info(),
                            system_program.to_account_info(),
                        ],
                    )?;
                }
                anchor_lang::solana_program::program::invoke_signed(
                    &anchor_lang::solana_program::system_instruction::allocate(
                        counter.to_account_info().key,
                        space as u64,
                    ),
                    &[counter.to_account_info(), system_program.to_account_info()],
                    &[&[b"counter".as_ref(), &[bump]][..]],
                )?;
                anchor_lang::solana_program::program::invoke_signed(
                    &anchor_lang::solana_program::system_instruction::assign(
                        counter.to_account_info().key,
                        program_id,
                    ),
                    &[counter.to_account_info(), system_program.to_account_info()],
                    &[&[b"counter".as_ref(), &[bump]][..]],
                )?;
            }
            let pa: anchor_lang::Account<Counter> =
                anchor_lang::Account::try_from_unchecked(&counter)?;
            pa
        };
        let (__program_signer, __bump) =
            anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
                &[b"counter".as_ref()],
                program_id,
            );
        if counter.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if __bump != bump {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !counter.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !__anchor_rent.is_exempt(
            counter.to_account_info().lamports(),
            counter.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
        }
        Ok(Create {
            counter,
            user,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for Create<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.counter.to_account_infos());
        account_infos.extend(self.user.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for Create<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.counter.to_account_metas(None));
        account_metas.extend(self.user.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for Create<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.counter, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_create {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct Create {
        pub counter: anchor_lang::solana_program::pubkey::Pubkey,
        pub user: anchor_lang::solana_program::pubkey::Pubkey,
        pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for Create
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.counter, writer)?;
            borsh::BorshSerialize::serialize(&self.user, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for Create {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.counter,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.user, true,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.system_program,
                    false,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// `cpi::accounts` module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_create {
    use super::*;
    pub struct Create<'info> {
        pub counter: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub user: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Create<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.counter),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.user),
                    true,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.system_program),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Create<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.counter));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.user));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.system_program,
            ));
            account_infos
        }
    }
}
# [instruction (_bump : u8)]
pub struct Increment<'info> {
    # [account (mut , seeds = [b"counter" . as_ref ()] , bump = counter . bump)]
    pub counter: Account<'info, Counter>,
    # [account (init , seeds = [user . key () . as_ref ()] , bump = _bump , payer = user)]
    pub has_incremented: Account<'info, HasIncremented>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for Increment<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let mut ix_data = ix_data;
        struct __Args {
            _bump: u8,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self._bump, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for __Args
        where
            u8: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    _bump: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        let __Args { _bump } = __Args::deserialize(&mut ix_data)
            .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
        let counter: anchor_lang::Account<Counter> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let has_incremented = &accounts[0];
        *accounts = &accounts[1..];
        let user: Signer = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let system_program: anchor_lang::Program<System> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let __anchor_rent = Rent::get()?;
        let has_incremented = {
            let space = 8 + HasIncremented::default().try_to_vec().unwrap().len();
            let payer = user.to_account_info();
            let __current_lamports = has_incremented.to_account_info().lamports();
            if __current_lamports == 0 {
                let lamports = __anchor_rent.minimum_balance(space);
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
            } else {
                let required_lamports = __anchor_rent
                    .minimum_balance(space)
                    .max(1)
                    .saturating_sub(__current_lamports);
                if required_lamports > 0 {
                    anchor_lang::solana_program::program::invoke(
                        &anchor_lang::solana_program::system_instruction::transfer(
                            payer.to_account_info().key,
                            has_incremented.to_account_info().key,
                            required_lamports,
                        ),
                        &[
                            payer.to_account_info(),
                            has_incremented.to_account_info(),
                            system_program.to_account_info(),
                        ],
                    )?;
                }
                anchor_lang::solana_program::program::invoke_signed(
                    &anchor_lang::solana_program::system_instruction::allocate(
                        has_incremented.to_account_info().key,
                        space as u64,
                    ),
                    &[
                        has_incremented.to_account_info(),
                        system_program.to_account_info(),
                    ],
                    &[&[user.key().as_ref(), &[_bump]][..]],
                )?;
                anchor_lang::solana_program::program::invoke_signed(
                    &anchor_lang::solana_program::system_instruction::assign(
                        has_incremented.to_account_info().key,
                        program_id,
                    ),
                    &[
                        has_incremented.to_account_info(),
                        system_program.to_account_info(),
                    ],
                    &[&[user.key().as_ref(), &[_bump]][..]],
                )?;
            }
            let pa: anchor_lang::Account<HasIncremented> =
                anchor_lang::Account::try_from_unchecked(&has_incremented)?;
            pa
        };
        let (__program_signer, __bump) =
            anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
                &[user.key().as_ref()],
                program_id,
            );
        if has_incremented.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if __bump != _bump {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !has_incremented.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !__anchor_rent.is_exempt(
            has_incremented.to_account_info().lamports(),
            has_incremented.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
        }
        let __program_signer =
            Pubkey::create_program_address(&[b"counter".as_ref(), &[counter.bump]][..], program_id)
                .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if counter.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !counter.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !user.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        Ok(Increment {
            counter,
            has_incremented,
            user,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for Increment<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.counter.to_account_infos());
        account_infos.extend(self.has_incremented.to_account_infos());
        account_infos.extend(self.user.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for Increment<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.counter.to_account_metas(None));
        account_metas.extend(self.has_incremented.to_account_metas(None));
        account_metas.extend(self.user.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for Increment<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.counter, program_id)?;
        anchor_lang::AccountsExit::exit(&self.has_incremented, program_id)?;
        anchor_lang::AccountsExit::exit(&self.user, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_increment {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct Increment {
        pub counter: anchor_lang::solana_program::pubkey::Pubkey,
        pub has_incremented: anchor_lang::solana_program::pubkey::Pubkey,
        pub user: anchor_lang::solana_program::pubkey::Pubkey,
        pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for Increment
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.counter, writer)?;
            borsh::BorshSerialize::serialize(&self.has_incremented, writer)?;
            borsh::BorshSerialize::serialize(&self.user, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for Increment {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.counter,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.has_incremented,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.user, true,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.system_program,
                    false,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// `cpi::accounts` module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_increment {
    use super::*;
    pub struct Increment<'info> {
        pub counter: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub has_incremented: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub user: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Increment<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.counter),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.has_incremented),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.user),
                true,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.system_program),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Increment<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.counter));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.has_incremented,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.user));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.system_program,
            ));
            account_infos
        }
    }
}
pub struct Counter {
    pub count: u64,
    pub bump: u8,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for Counter {
    #[inline]
    fn default() -> Counter {
        Counter {
            count: ::core::default::Default::default(),
            bump: ::core::default::Default::default(),
        }
    }
}
impl borsh::ser::BorshSerialize for Counter
where
    u64: borsh::ser::BorshSerialize,
    u8: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.count, writer)?;
        borsh::BorshSerialize::serialize(&self.bump, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for Counter
where
    u64: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            count: borsh::BorshDeserialize::deserialize(buf)?,
            bump: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Counter {
    #[inline]
    fn clone(&self) -> Counter {
        match *self {
            Counter {
                count: ref __self_0_0,
                bump: ref __self_0_1,
            } => Counter {
                count: ::core::clone::Clone::clone(&(*__self_0_0)),
                bump: ::core::clone::Clone::clone(&(*__self_0_1)),
            },
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for Counter {
    fn try_serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::result::Result<(), ProgramError> {
        writer
            .write_all(&[255, 176, 4, 245, 188, 253, 124, 25])
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        AnchorSerialize::serialize(self, writer)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for Counter {
    fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        if buf.len() < [255, 176, 4, 245, 188, 253, 124, 25].len() {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[255, 176, 4, 245, 188, 253, 124, 25] != given_disc {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorMismatch.into());
        }
        Self::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        let mut data: &[u8] = &buf[8..];
        AnchorDeserialize::deserialize(&mut data)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotDeserialize.into())
    }
}
#[automatically_derived]
impl anchor_lang::Discriminator for Counter {
    fn discriminator() -> [u8; 8] {
        [255, 176, 4, 245, 188, 253, 124, 25]
    }
}
#[automatically_derived]
impl anchor_lang::Owner for Counter {
    fn owner() -> Pubkey {
        crate::ID
    }
}
pub struct HasIncremented {
    pub has_incremented: bool,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for HasIncremented {
    #[inline]
    fn default() -> HasIncremented {
        HasIncremented {
            has_incremented: ::core::default::Default::default(),
        }
    }
}
impl borsh::ser::BorshSerialize for HasIncremented
where
    bool: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.has_incremented, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for HasIncremented
where
    bool: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            has_incremented: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for HasIncremented {
    #[inline]
    fn clone(&self) -> HasIncremented {
        match *self {
            HasIncremented {
                has_incremented: ref __self_0_0,
            } => HasIncremented {
                has_incremented: ::core::clone::Clone::clone(&(*__self_0_0)),
            },
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for HasIncremented {
    fn try_serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::result::Result<(), ProgramError> {
        writer
            .write_all(&[56, 222, 56, 247, 106, 17, 240, 42])
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        AnchorSerialize::serialize(self, writer)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for HasIncremented {
    fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        if buf.len() < [56, 222, 56, 247, 106, 17, 240, 42].len() {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[56, 222, 56, 247, 106, 17, 240, 42] != given_disc {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorMismatch.into());
        }
        Self::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        let mut data: &[u8] = &buf[8..];
        AnchorDeserialize::deserialize(&mut data)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotDeserialize.into())
    }
}
#[automatically_derived]
impl anchor_lang::Discriminator for HasIncremented {
    fn discriminator() -> [u8; 8] {
        [56, 222, 56, 247, 106, 17, 240, 42]
    }
}
#[automatically_derived]
impl anchor_lang::Owner for HasIncremented {
    fn owner() -> Pubkey {
        crate::ID
    }
}
