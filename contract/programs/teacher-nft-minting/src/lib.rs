use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::{
    system_instruction,
};
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token,};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

pub mod merkle_proof;

declare_id!("2fikMtYFNn7gSANHrXQSDRXxVj3sEjvLjLAitg8614XH");
pub mod constants {
    pub const MINTING_PDA_SEED: &[u8] = b"teacher_minting";
}

#[program]
pub mod metaplex_anchor_nft {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        _nonce_minting: u8,
        authorized_creator: Pubkey,
        max_teacher: u64,
        og_max: u64,
        wl_max: u64,
        bl_max: u64,
        og_price: u64,
        wl_price: u64,
        bl_price: u64,
        cur_num: u64,
        cur_stage: u8,
        root: [u8; 32],
    ) -> ProgramResult {

        ctx.accounts.minting_account.admin_key = *ctx.accounts.initializer.key;
        ctx.accounts.minting_account.authorized_creator = authorized_creator;
        ctx.accounts.minting_account.max_teacher = max_teacher;
        ctx.accounts.minting_account.og_max = og_max;
        ctx.accounts.minting_account.wl_max = wl_max;
        ctx.accounts.minting_account.bl_max = bl_max;
        ctx.accounts.minting_account.og_price = og_price;
        ctx.accounts.minting_account.wl_price = wl_price;
        ctx.accounts.minting_account.bl_price = bl_price;
        ctx.accounts.minting_account.cur_num = cur_num;
        ctx.accounts.minting_account.cur_stage = cur_stage;
        ctx.accounts.minting_account.og_root = root;

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn update_admin(
        ctx: Context<UpdateAdmin>,
        _nonce_minting: u8,
        new_admin: Pubkey,
    ) -> ProgramResult {
        ctx.accounts.minting_account.admin_key = new_admin;

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn update_og_root(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        og_list: String,
        og_root_url: String,
        og_root_hash: [u8; 32]
        ) -> ProgramResult {
        ctx.accounts.minting_account.og_list_url = og_list;
        ctx.accounts.minting_account.og_root_url = og_root_url;
        ctx.accounts.minting_account.og_root = og_root_hash;
        Ok(())
    }

    pub fn is_og_list(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        proof: Vec<[u8; 32]>,
    ) -> ProgramResult {
        let new_account = &ctx.accounts.admin;
        let node = anchor_lang::solana_program::keccak::hashv(&[
            &new_account.key.to_string().as_ref()
        ]);
        if merkle_proof::verify(proof, ctx.accounts.minting_account.og_root, node.to_bytes()) == false {
            return Err(MyError::InvalidProof.into());
        }
        
        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn add_og_list(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        new_og_list: Vec<String>,
    ) -> ProgramResult { 
        for new_og in new_og_list.iter() {
            if ctx
                .accounts
                .minting_account
                .og_list
                .iter()
                .find(|&og| og == new_og)
                == None
            {
                ctx.accounts
                    .minting_account
                    .og_list
                    .push(new_og.to_string());
            }
        }

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn remove_og_list(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        old_og_list: Vec<String>,
    ) -> ProgramResult {
        for old_og in old_og_list.iter() {
            match ctx
                .accounts
                .minting_account
                .og_list
                .iter()
                .position(|og| {
                    og == old_og
                }) {
                Some(index) => {
                    ctx.accounts
                        .minting_account
                        .og_list
                        .remove(index);
                }
                None => {}
            }
        }

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn add_wl_list(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        new_og_list: Vec<String>,
    ) -> ProgramResult {
        for new_og in new_og_list.iter() {
            if ctx
                .accounts
                .minting_account
                .wl_list
                .iter()
                .find(|&og| og == new_og)
                == None
            {
                ctx.accounts
                    .minting_account
                    .wl_list
                    .push(new_og.to_string());
            }
        }

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn remove_wl_list(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        old_og_list: Vec<String>,
    ) -> ProgramResult {
        for old_og in old_og_list.iter() {
            match ctx
                .accounts
                .minting_account
                .wl_list
                .iter()
                .position(|og| {
                    og == old_og
                }) {
                Some(index) => {
                    ctx.accounts
                        .minting_account
                        .wl_list
                        .remove(index);
                }
                None => {}
            }
        }

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn add_bl_list(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        new_og_list: Vec<String>,
    ) -> ProgramResult {
        for new_og in new_og_list.iter() {
            if ctx
                .accounts
                .minting_account
                .bl_list
                .iter()
                .find(|&og| og == new_og)
                == None
            {
                ctx.accounts
                    .minting_account
                    .bl_list
                    .push(new_og.to_string());
            }
        }

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn remove_bl_list(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        old_og_list: Vec<String>,
    ) -> ProgramResult {
        for old_og in old_og_list.iter() {
            match ctx
                .accounts
                .minting_account
                .bl_list
                .iter()
                .position(|og| {
                    og == old_og
                }) {
                Some(index) => {
                    ctx.accounts
                        .minting_account
                        .bl_list
                        .remove(index);
                }
                None => {}
            }
        }

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn update_price(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        new_og_price: u64,
        new_wl_price: u64,
        new_bl_price: u64,
    ) -> ProgramResult {

        if new_og_price > 0 {
            ctx.accounts.minting_account.og_price = new_og_price;    
        }
        if new_wl_price > 0 {
            ctx.accounts.minting_account.wl_price = new_wl_price;    
        }
        if new_bl_price > 0 {
            ctx.accounts.minting_account.bl_price = new_bl_price;    
        }

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn update_amount(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        new_og_amout: u64,
        new_wl_amout: u64,
        new_bl_amout: u64,
    ) -> ProgramResult {

        if new_og_amout > 0 {
            ctx.accounts.minting_account.og_max = new_og_amout;    
        }
        if new_wl_amout > 0 {
            ctx.accounts.minting_account.wl_max = new_wl_amout;    
        }
        if new_bl_amout > 0 {
            ctx.accounts.minting_account.bl_max = new_bl_amout;    
        }

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn set_stage(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        new_stage: u8,
    ) -> ProgramResult {

        if new_stage > 0 && new_stage < 4 {
            ctx.accounts.minting_account.cur_stage = new_stage;    
        }
        // 1 => OG; 2 => WL; 3 => BL; 5(other) => Public;
        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn set_uri(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        new_uri: String,
    ) -> ProgramResult {
        ctx.accounts.minting_account.base_uri = new_uri;    
        Ok(())
    }

   pub fn mint_nft(
        ctx: Context<MintNFT>,
        creator_key: Pubkey,
        title: String,
        proof: Vec<[u8; 32]>,
    ) -> ProgramResult {
            // set user minting info
        let mut _max_num = ctx.accounts.minting_account.bl_max;
        let mut _price = ctx.accounts.minting_account.bl_price;
        let mut _state = 3;
        
        let new_account = &ctx.accounts.mint_authority;
        let node = anchor_lang::solana_program::keccak::hashv(&[
            &new_account.key.to_string().as_ref()
        ]);
        if merkle_proof::verify(proof, ctx.accounts.minting_account.og_root, node.to_bytes()) {
            _max_num = ctx.accounts.minting_account.og_max;
            _price = ctx.accounts.minting_account.og_price;
            _state = 1;
        }

        // match ctx.accounts.minting_account.og_list.iter().position(|og| { *og == ctx.accounts.payer.key().to_string() }) {
        //         Some(_index) => {
        //             _max_num = ctx.accounts.minting_account.og_max;
        //             _price = ctx.accounts.minting_account.og_price;
        //             _state = 1;
        //         }
        //         None => {}
        //     } 
        match ctx.accounts.minting_account.wl_list.iter().position(|og| { *og == ctx.accounts.payer.key.key().to_string() }) {
                Some(_index) => {
                    _max_num = ctx.accounts.minting_account.wl_max;
                    _price = ctx.accounts.minting_account.wl_price;
                    _state = 2;
                }
                None => {}
            } 
        match ctx.accounts.minting_account.bl_list.iter().position(|og| { *og == ctx.accounts.payer.key.key().to_string() }) {
                Some(_index) => {
                    _state = 5;
                }
                None => {}
            } 
        
        if ctx.accounts.minting_account.cur_stage != 3 { 
            if ctx.accounts.minting_account.max_teacher <= ctx.accounts.minting_account.cur_num
                || ctx.accounts.minting_account.cur_stage != _state || ctx.accounts.user_minting_counter_account.cur_num >= _max_num {
                return Err(MyError::InvalidOperation.into());
            }
        }

        // if ctx.accounts.minting_account.admin_key != *ctx.accounts.owner.key {
        //     return Err(MyError::InvalidOperation.into());
        // }

        let transfer_sol_to_seller = system_instruction::transfer(
            ctx.accounts.payer.key,
            ctx.accounts.owner.key,
            _price,
        );
        

        invoke(
            &transfer_sol_to_seller,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
            ],
        )?;

        msg!("Initializing Mint Ticket");
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        msg!("CPI Accounts Assigned");
        let cpi_program = ctx.accounts.token_program.to_account_info();
        msg!("CPI Program Assigned");
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        msg!("CPI Context Assigned");
        token::mint_to(cpi_ctx, 1)?;
        msg!("Token Minted !!!");
        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Account Info Assigned");
        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: ctx.accounts.mint_authority.key(),
                verified: false,
                share: 0,
            },
        ];
        let new_uri = format!("{}{}{}",ctx.accounts.minting_account.base_uri, ctx.accounts.minting_account.cur_num , ".json");
        msg!("Creator Assigned");
        let symbol = std::string::ToString::to_string("symb");
        invoke(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                title,
                symbol,
                new_uri,
                Some(creator),
                1,
                true,
                false,
                None,
                None,
            ),
            account_info.as_slice(),
        )?;
        msg!("Metadata Account Created !!!");
        let master_edition_infos = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Master Edition Account Infos Assigned");
        invoke(
            &create_master_edition_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.master_edition.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.payer.key(),
                Some(0),
            ),
            master_edition_infos.as_slice(),
        )?;
        msg!("Master Edition Nft Minted !!!");
        ctx.accounts.user_minting_counter_account.cur_num += 1;
        ctx.accounts.minting_account.cur_num += 1;
        Ok(())
    }

}
#[derive(Accounts)]
#[instruction(_nonce_minting: u8)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        payer = initializer,
        seeds = [ constants::MINTING_PDA_SEED.as_ref() ],
        bump = _nonce_minting,
        space = 32 * 10 + 32 * 3 * 50
    )]
    pub minting_account: Box<Account<'info, MintingAccount>>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(Default)]
pub struct MintingAccount {
    pub admin_key: Pubkey,
    pub freeze_program: bool,
    pub aury_vault: Pubkey,
    pub authorized_creator: Pubkey,
    pub max_teacher: u64,
    pub og_max: u64,
    pub wl_max: u64,
    pub bl_max: u64,
    pub og_price: u64,
    pub wl_price: u64,
    pub bl_price: u64,
    pub og_list: Vec<String>,
    pub wl_list: Vec<String>,
    pub bl_list: Vec<String>,    
    pub og_root: [u8; 32],
    pub og_list_url: String,
    pub og_root_url: String,
    pub cur_num: u64,
    pub cur_stage: u8,
    pub base_uri: String,
}
#[derive(Accounts)]
#[instruction(_nonce_minting: u8)]
pub struct CommonSt<'info> {
    #[account(
        mut,
        seeds = [ constants::MINTING_PDA_SEED.as_ref() ],
        bump = _nonce_minting,
    )]
    pub minting_account: Box<Account<'info, MintingAccount>>,

    pub admin: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct UserMintingAccount {
    pub cur_num: u64,
}

#[derive(Accounts)]
#[instruction(_nonce_minting: u8)]
pub struct UpdateAdmin<'info> {
    #[account(
        mut,
        seeds = [ constants::MINTING_PDA_SEED.as_ref() ],
        bump = _nonce_minting,
    )]
    pub minting_account: Box<Account<'info, MintingAccount>>,

    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    // #[account(mut)]
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
     #[account(
        mut,
        seeds = [ constants::MINTING_PDA_SEED.as_ref() ],
        bump,
        constraint = !minting_account.freeze_program,
    )]
    pub minting_account: Box<Account<'info, MintingAccount>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [ payer.key().as_ref() ],
        bump,
    )]
    pub user_minting_counter_account: Box<Account<'info, UserMintingAccount>>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
}
#[error]
pub enum MyError {
    #[msg("This error occur as owner strategy.")]
    InvalidOperation,
    #[msg("Invalid Merkle proof.")]
    InvalidProof,
}
fn is_admin<'info>(
    minting_account: &Account<'info, MintingAccount>,
    signer: &Signer<'info>,
) -> ProgramResult {
    if minting_account.admin_key != *signer.key {
        return Err(MyError::InvalidOperation.into());
    }

    Ok(())
}