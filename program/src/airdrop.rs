use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke},
    program_error::ProgramError,
    pubkey::Pubkey
};

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

    pub fn tokens_available(
        account: &AccountInfo
    ) -> u64 {
        account.lamports()
    }
}
