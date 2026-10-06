//! Нативная программа с CPI в System Program.
//! Инструкция 0: transfer(lamports) от подписанта, обычный `invoke`.
//! Инструкция 1: transfer(lamports) из PDA [b"vault", bump], `invoke_signed`.
//! Формат данных: [opcode: u8, lamports: u64 LE].

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_system_interface::instruction as system_instruction;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (&op, rest) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    let lamports = rest
        .get(..8)
        .map(|b| u64::from_le_bytes(b.try_into().unwrap()))
        .ok_or(ProgramError::InvalidInstructionData)?;

    let iter = &mut accounts.iter();
    let from = next_account_info(iter)?;
    let to = next_account_info(iter)?;
    let system_program = next_account_info(iter)?;

    match op {
        0 => {
            if !from.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }
            invoke(
                &system_instruction::transfer(from.key, to.key, lamports),
                &[from.clone(), to.clone(), system_program.clone()],
            )
        }
        1 => {
            let (vault, bump) = Pubkey::find_program_address(&[b"vault"], program_id);
            if vault != *from.key {
                return Err(ProgramError::InvalidSeeds);
            }
            msg!("vault transfer {}", lamports);
            invoke_signed(
                &system_instruction::transfer(from.key, to.key, lamports),
                &[from.clone(), to.clone(), system_program.clone()],
                &[&[b"vault", &[bump]]],
            )
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
