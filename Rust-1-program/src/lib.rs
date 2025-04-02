use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{next_account_info,AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::PubKey,
    entrypoint
}

enum InstructionType {
    Increment(u32),
    Decrement(u32)
}

struct Counter {
   count:u32
}

entrypoint!(counter);
pub fn counter(
    program_id: &PubKey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
)-> ProgramResult{
    let account = next_account_info(&mut accounts.iter())?;

    let instruction_type = InstructionType::try_from_slice(instruction_data)?;

    let mut counter_data = Counter::try_from_slice(&account.data.borrow())?;
    match instruction_type {
        InstructionType::Increment(value)=>{
            counter_data.count += value;
        },
        InstructionType::Decrement(value)=>{
            counter_data.count += value;
        }

    }

    counter_data.serialize(&mut *account.data.borrow_mut());

    msg!("Counter updated to {} ", counter_data.count);

    Ok(())
}