use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke},
    program_error::ProgramError,
    pubkey::Pubkey
};

use spl_associated_token_account::create_associated_token_account;
use spl_token::instruction::transfer_checked;

pub struct Airdrop;
impl Airdrop {
    pub fn start(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {

        let account_info_iter = &mut accounts.iter();
        // the owner of the tokens for the airdrop
        let token_program = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;

        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let (_pda, _nonce) = Pubkey::find_program_address(&[b"escrow"], program_id);

        while Self::tokens_available(owner) > 0 {
            let receipient = next_account_info(account_info_iter)?;
            // transfer the token to receipient
            let transfer_to_receipient_ix = spl_token::instruction::transfer(
                token_program.key,
                owner.key,
                receipient.key,
                owner.key,
                &[&owner.key],
                amount,
            )?;
            msg!("Calling the token program to transfer tokens to receipient...");
            invoke(
                &transfer_to_receipient_ix,
                &[
                    owner.clone(),
                    receipient.clone(),
                    token_program.clone(),
                ],
            )?;
        }

        Ok(())
    }

    pub fn transfer_nft(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let token_program = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;
        let wallet_nft = next_account_info(account_info_iter)?;
        let mint_nft = next_account_info(account_info_iter)?;
        let from_nft = next_account_info(account_info_iter)?;
        let to_nft = next_account_info(account_info_iter)?;

        if !owner.is_signer {
            return Err(ProgramError::InvalidSeeds);
        }

        let get_tag = instruction_data
            .last()
            .map(|value| value.to_string())
            .unwrap()
            .parse::<u64>();

        let tag = match get_tag {
            Ok(tag) => tag,
            Err(_) => return Err(ProgramError::InvalidInstructionData),
        };

        if tag == 0 {
            let associated_token_program_account = next_account_info(account_info_iter)?;
            let system_program_account = next_account_info(account_info_iter)?;
            let sysvar_rent_account = next_account_info(account_info_iter)?;

            let create_token_account_instruction =
                create_associated_token_account(&owner.key, &owner.key, &mint_nft.key);

            invoke(
                &create_token_account_instruction,
                &[
                    associated_token_program_account.clone(),
                    owner.clone(),
                    to_nft.clone(),
                    mint_nft.clone(),
                    system_program_account.clone(),
                    token_program.clone(),
                    sysvar_rent_account.clone(),
                ],
            )?;
        }

        let transfer_nft_instruction = transfer_checked(
            &token_program.key,
            &from_nft.key,
            &mint_nft.key,
            &to_nft.key,
            &wallet_nft.key,
            &[&wallet_nft.key],
            1,
            0,
        )?;

        msg!("Calling the token program to transfer nft...");

        invoke(
            &transfer_nft_instruction,
            &[
                token_program.clone(),
                from_nft.clone(),
                mint_nft.clone(),
                to_nft.clone(),
                wallet_nft.clone(),
            ],
        )?;

        Ok(())
    }

    pub fn tokens_available(
        account: &AccountInfo
    ) -> u64 {
        account.lamports()
    }
}
