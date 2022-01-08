use solana_program::{program_error::ProgramError};

#[derive(Debug)]
pub enum GreetingInstruction {
    SayHello {
        age: u8,
        name: String,
    },
    SayBye {
        age: u8,
        name: String,
    },
}

// instruction.rs is responsibe for decoding the instruction_data
impl GreetingInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // First byte is one of 256 possible operations to index since a byte can take values 0-255
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        // Retrun GreetingInstruction wrapped in Result variant Ok, or error wrapped in Err variant
        Ok(match tag {
                0 => {
                    let (&age, rest) = rest.split_first().ok_or(ProgramError::InvalidInstructionData)?;
                    Self::SayHello {
                        age,
                        name: std::str::from_utf8(rest).to_owned().unwrap().trim().to_string(),
                    }
                },
                1 =>  {
                    let (&age, rest) = rest.split_first().ok_or(ProgramError::InvalidInstructionData)?;
                    Self::SayBye {
                        age,
                        name: std::str::from_utf8(rest).to_owned().unwrap().trim().to_string(),
                    }
                },
                _ => return Err(ProgramError::InvalidInstructionData),
            },
        )
    }
}