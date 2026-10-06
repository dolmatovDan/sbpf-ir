//! Простая нативная программа: ветвления, 32/64-битная арифметика, деление, циклы, логи.
//! Формат данных инструкции: [opcode: u8, a: u64 LE, b: u64 LE].

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn read_u64(data: &[u8], offset: usize) -> Result<u64, ProgramError> {
    data.get(offset..offset + 8)
        .map(|b| u64::from_le_bytes(b.try_into().unwrap()))
        .ok_or(ProgramError::InvalidInstructionData)
}

#[inline(never)]
fn alu64(a: u64, b: u64) -> u64 {
    let x = a.wrapping_mul(b) ^ (a >> 3);
    let y = (b as i64).wrapping_shr(2) as u64;
    x.wrapping_add(y).rotate_left(7)
}

#[inline(never)]
fn alu32(a: u32, b: u32) -> u32 {
    a.wrapping_add(b).wrapping_mul(3) ^ (a << 5) | (b >> 1)
}

#[inline(never)]
fn checked_div(a: u64, b: u64) -> Result<u64, ProgramError> {
    a.checked_div(b).ok_or(ProgramError::ArithmeticOverflow)
}

#[inline(never)]
fn signed_cmp(a: i64, b: i64) -> i64 {
    if a < b {
        -1
    } else if a > b {
        1
    } else {
        0
    }
}

#[inline(never)]
fn sum_loop(n: u64) -> u64 {
    let mut acc = 0u64;
    let mut i = 0u64;
    while i < n {
        acc = acc.wrapping_add(i * i);
        i += 1;
    }
    acc
}

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (&op, _) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    let a = read_u64(data, 1)?;
    let b = read_u64(data, 9)?;

    let result = match op {
        0 => alu64(a, b),
        1 => alu32(a as u32, b as u32) as u64,
        2 => checked_div(a, b)?,
        3 => signed_cmp(a as i64, b as i64) as u64,
        4 => sum_loop(a.min(1000)),
        5 => (a as i32 as i64 % (b as i32 as i64).max(1)) as u64,
        _ => return Err(ProgramError::InvalidInstructionData),
    };
    msg!("op {} -> {}", op, result);
    Ok(())
}
