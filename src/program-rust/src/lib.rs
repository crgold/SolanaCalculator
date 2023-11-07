use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
pub mod instruction;
use instruction::{CalculatorInstruction};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalculatorAccount {
    /// number of greetings
    pub result: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the calculator program was loaded into
    accounts: &[AccountInfo], // The account to store result in
    _instruction_data: &[u8],
) -> ProgramResult {

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account where result will be stored
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Calculator account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // get instruction provided by user
    let instruction = CalculatorInstruction::unpack(_instruction_data)?;

    // get users CalculatorAccount
    let mut calculator_account = CalculatorAccount::try_from_slice(&account.data.borrow())?;

    // use a match to determine the operation entered by the user
    match instruction {
        CalculatorInstruction::Add {val1, val2} => {
            calculator_account.result = val1 + val2;
            calculator_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

            msg!("Added {} and {} which resulted in a result of {}", val1, val2, calculator_account.result);

            Ok(())
        },
        CalculatorInstruction::Subtract {val1, val2} => {
            calculator_account.result = val1 - val2;
            calculator_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

            msg!("Subtracted {} from {} which resulted in a result of {}", val1, val2, calculator_account.result);

            Ok(())
        }
    }
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}
