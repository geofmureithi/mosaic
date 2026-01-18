use crate::instructions::{
    Instruction, execute::Execute, init_root::InitializeOperators,
    init_signing_session::InitializeSigningSession, sign::Sign,
};
use pinocchio::{AccountView, Address, ProgramResult, error::ProgramError};

pub fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    mandatory_checks(program_id)?;

    let (opcode, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match Instruction::try_from(opcode)? {
        Instruction::InitializeOperators => {
            InitializeOperators::try_from((accounts, data))?.handler()
        }
        Instruction::InitializeSigningSession => {
            InitializeSigningSession::try_from((accounts, data))?.handler()
        }
        Instruction::Sign => Sign::try_from((accounts, data))?.handler(),
        Instruction::Execute => Execute::try_from((accounts, data))?.handler(),
    }
}

#[must_use]
fn mandatory_checks(program_id: &Address) -> Result<(), ProgramError> {
    if program_id != &crate::ID.into() {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}
