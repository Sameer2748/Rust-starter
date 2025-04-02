use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError
};

/// Define instruction types with serialization support
#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

/// Define Counter struct with serialization
#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Counter {
    count: u32,
}

entrypoint!(counter);

/// Entry point for the program
pub fn counter(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account = next_account_info(&mut accounts.iter())?;

    // Deserialize instruction type
    let instruction_type: InstructionType = match InstructionType::try_from_slice(instruction_data) {
        Ok(instruction) => instruction,
        Err(_) => return Err(ProgramError::InvalidInstructionData),
    };

    // Deserialize account data into counter
    let mut counter_data: Counter = match Counter::try_from_slice(&account.data.borrow()) {
        Ok(data) => data,
        Err(_) => return Err(ProgramError::InvalidAccountData),
    };

    // Process instructions
    match instruction_type {
        InstructionType::Increment(value) => {
            counter_data.count += value;
        }
        InstructionType::Decrement(value) => {
            counter_data.count = counter_data.count.saturating_sub(value); // Prevents underflow
        }
    }

    // Serialize and store updated counter data
    counter_data.serialize(&mut *account.data.borrow_mut())?;

    msg!("Counter updated to {} ", counter_data.count);

    Ok(())
}
