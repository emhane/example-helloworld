use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Bring HelloInstruction into scope
pub mod instruction;
use crate::instruction::GreetingInstruction;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
    /// age as sent in instruction_data
    pub age: u32,
    /// for this simple demo only datatypes of known size are stored that can be implicitly de-/serialized
    pub first_letter: u8,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Unpack instruction and log it
    let instruction = GreetingInstruction::unpack(instruction_data)?;
    msg!("Instruction: {:?}", instruction);

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        GreetingInstruction::SayHello { age, name } => {
            // increment counter for SayHello variant
            greeting_account.counter += 1;
            greeting_account.age = u32::from(age);
            greeting_account.first_letter = name.as_bytes()[0];
            msg!("Hello! Age: {:?}, Name: {:?}", age.to_string(), &name);
        },
        GreetingInstruction::SayBye { age, name } => {
            // decrease counter for SayBye variant
            greeting_account.counter -= 1;
            greeting_account.age = u32::from(age);
            greeting_account.first_letter = name.as_bytes()[0];
            msg!("Bye! Age: {:?}, Name: {:?}", age.to_string(), &name);
        }
    }  

    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; 9];
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
        let instruction_data: Vec<u8> = vec![0, 0, 0];
        let instruction_data_bye: Vec<u8> = vec![1, 0, 0];

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
        process_instruction(&program_id, &accounts, &instruction_data_bye).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );
    }
}
