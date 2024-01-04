use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::{next_account_info,AccountInfo},
    system_instruction,
    program_error::ProgramError,
    sysvar::{rent::Rent, Sysvar},
    program::{invoke_signed},
    borsh::try_from_slice_unchecked,
};
use std::convert::TryInto;
pub mod instruction;
pub mod state;
use instruction::MovieInstruction;
use state::MovieAccountState;
use borsh::BorshSerialize;


entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    
    let instruction = MovieInstruction::unpack(instruction_data)?;

    match instruction {
        MovieInstruction::AddMovieReview {title, rating, description} => {
            add_movie_review(program_id, accounts, title, rating, description);
        }
    }
    Ok(())
}

pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String
) -> ProgramResult {

    msg!("adding movie review...");
    msg!("Title :{}", title);
    msg!("Rating :{}", rating);
    msg!("Description :{}", description);

    // 迭代账户
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // 衍生PDA
    let (pad, bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref(), title.as_bytes().as_ref()], program_id);

    // 计算占用空间和租金
    let account_len: usize = 1 + 1 + (4 + title.len()) + (4 + description.len());
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    // 创建新账户
    invoke_signed(
        &system_instruction::create_account(
            initializer.key, 
            pda_account.key, 
            rent_lamports, 
            account_len.try_into().unwrap(), 
            program_id
            ), 
            &[initializer.clone(), pda_account.clone(), system_program.clone()], 
            &[&[initializer.key.as_ref(), title.as_bytes().as_ref(), &[bump_seed]]],
        )?;
    msg!("PDA created: {}", pad);

    // 更新数据账户
    msg!("unpacking state account");
    let mut account_data = try_from_slice_unchecked::<MovieAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    account_data.title = title;
    account_data.rating = rating;
    account_data.description = description;
    account_data.is_initialized = true;

    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");
    
    msg!("add state sucess!");
    Ok(())
}