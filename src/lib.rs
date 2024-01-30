use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::instruction::CounterInstructions;

pub mod instruction;
pub mod testing;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {
    pub counter: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instructions = CounterInstructions::unpack(instruction_data)?;

    let accounts_iter = &mut accounts.iter();

    let account = next_account_info(accounts_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instructions {
        CounterInstructions::Increment(args) => {
            counter_account.counter += args.value;
            msg!("This is the value: {}", counter_account.counter);
        }
        CounterInstructions::Decrement(args) => {
            counter_account.counter -= args.value;
            msg!("This is the value: {}", counter_account.counter);
        }
        CounterInstructions::Update(args) => {
            counter_account.counter = args.value;
            msg!("This is the value: {}", counter_account.counter);
        }
        CounterInstructions::Reset => {
            counter_account.counter = 0;
            msg!("This is the value: {}", counter_account.counter);
        }
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}
