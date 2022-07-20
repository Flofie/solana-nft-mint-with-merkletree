use anchor_lang::prelude::*;
use mpl_token_metadata::{instruction::set_and_verify_collection, utils::assert_derivation};
use solana_program::{
    program::invoke_signed, sysvar, sysvar::instructions::get_instruction_relative,
};


/// Collection PDA account
#[account]
#[derive(Default, Debug)]
pub struct CollectionState {
    pub mint: Pubkey,
    pub authority: Pubkey,
}

/// Sets and verifies the collection during a candy machine mint
#[derive(Accounts)]
pub struct SetCollectionDuringMint<'info> {
    /// CHECK: account checked in CPI/instruction sysvar
    metadata: UncheckedAccount<'info>,
    payer: Signer<'info>,
    #[account(mut, seeds = [b"collection".as_ref()], bump)]
    collection_pda: Account<'info, CollectionState>,
    /// CHECK: account constraints checked in account trait
    #[account(address = mpl_token_metadata::id())]
    token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: account constraints checked in account trait
    #[account(address = sysvar::instructions::id())]
    instructions: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    collection_mint: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    collection_metadata: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    collection_master_edition: UncheckedAccount<'info>,
    /// CHECK: authority can be any account and is checked in CPI
    authority: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    collection_authority_record: UncheckedAccount<'info>,
}

pub fn handle_set_collection_during_mint(ctx: Context<SetCollectionDuringMint>) -> Result<()> {
    let ixs = &ctx.accounts.instructions;
    let previous_instruction = get_instruction_relative(-1, ixs)?;

    let discriminator = &previous_instruction.data[0..8];
    if discriminator != [211, 57, 6, 167, 15, 219, 35, 251] {
        msg!("Transaction had ix with data {:?}", discriminator);
        return Ok(());
    }

    // let mint_ix_accounts = previous_instruction.accounts;
    // let mint_ix_cm = mint_ix_accounts[0].pubkey;
    // let mint_ix_metadata = mint_ix_accounts[4].pubkey;
    // let signer = mint_ix_accounts[6].pubkey;
    // let metadata = ctx.accounts.metadata.key();
    // let payer = ctx.accounts.payer.key();

    let collection_pda = &ctx.accounts.collection_pda;
    let collection_mint = ctx.accounts.collection_mint.to_account_info();

    let seeds = [b"collection".as_ref()];
    let bump = assert_derivation(&crate::id(), &collection_pda.to_account_info(), &seeds)?;
    let signer_seeds = [b"collection".as_ref(), &[bump]];
    let set_collection_infos = vec![
        ctx.accounts.metadata.to_account_info(),
        collection_pda.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        collection_mint.to_account_info(),
        ctx.accounts.collection_metadata.to_account_info(),
        ctx.accounts.collection_master_edition.to_account_info(),
        ctx.accounts.collection_authority_record.to_account_info(),
    ];
    invoke_signed(
        &set_and_verify_collection(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            collection_pda.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.authority.key(),
            collection_mint.key(),
            ctx.accounts.collection_metadata.key(),
            ctx.accounts.collection_master_edition.key(),
            Some(ctx.accounts.collection_authority_record.key()),
        ),
        set_collection_infos.as_slice(),
        &[&signer_seeds],
    )?;
    Ok(())
}
