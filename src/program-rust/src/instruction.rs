use borsh::{BorshDeserialize};
use solana_program::{program_error::ProgramError};

#[derive(BorshDeserialize)]
struct CalculatorInput {
    val1: u32,
    val2: u32
}

// create enum to represent the various instructions that a user can pass in
pub enum CalculatorInstruction{
    Add{
        val1: u32,
        val2: u32
    },
    Subtract {
        val1: u32,
        val2: u32
    }
}

impl CalculatorInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // get data from user
        let (&instruct, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        let user_data = CalculatorInput::try_from_slice(rest).unwrap();
        Ok(match instruct {
            0 => Self::Add {
                val1: user_data.val1,
                val2: user_data.val2
            },
            1 => Self::Subtract {
                val1: user_data.val1,
                val2: user_data.val2
            },
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}