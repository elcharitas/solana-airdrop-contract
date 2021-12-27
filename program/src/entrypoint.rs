use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

use crate::processor::Airdrop;

entrypoint!(process_airdrop);
fn process_airdrop(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    extras: &[u8],
) -> ProgramResult {
    //let (amount, _rest) = extras.split_first().ok_or("Amount is required!").unwrap();
    Airdrop::transfer_nft(program_id, accounts, extras)
}
