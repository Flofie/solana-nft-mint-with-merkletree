use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};
use std::mem::size_of;

pub mod merkle_proof;

declare_id!("EKsCmywFjtZ6DABbi57qvqK2CBMjFsC7c3CXV3G2HfKs");
pub mod constants {
    pub const MINTING_PDA_SEED: &[u8] = b"wallet_mint";
    pub const NFT_CREATOR_SEED: &str = "NFT_CREATOR_SEED";
}

#[program]
pub mod metaplex_anchor_nft {

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        max_supply: u64,
        og_max: u64,
        wl_max: u64,
        public_max: u64,
        og_price: u64,
        wl_price: u64,
        public_price: u64,
        title: String,
        symbol: String,
        base_uri: String,
    ) -> Result<()> {
        ctx.accounts.minting_account.admin_key = *ctx.accounts.initializer.key;
        ctx.accounts.minting_account.max_supply = max_supply;
        ctx.accounts.minting_account.og_max = og_max;
        ctx.accounts.minting_account.wl_max = wl_max;
        ctx.accounts.minting_account.public_max = public_max;
        ctx.accounts.minting_account.og_price = og_price;
        ctx.accounts.minting_account.wl_price = wl_price;
        ctx.accounts.minting_account.public_price = public_price;
        ctx.accounts.minting_account.cur_num = 0;
        ctx.accounts.minting_account.cur_stage = 0;
        ctx.accounts.minting_account.base_uri = base_uri;
        ctx.accounts.minting_account.base_title = title;
        ctx.accounts.minting_account.symbol = symbol;
        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn update_admin(
        ctx: Context<UpdateAdmin>,
        _nonce_minting: u8,
        new_admin: Pubkey,
    ) -> Result<()> {
        ctx.accounts.minting_account.admin_key = new_admin;

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn update_og_root(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        og_list: String,
        og_root_url: String,
        og_root_hash: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.minting_account.og_list_url = og_list;
        ctx.accounts.minting_account.og_root_url = og_root_url;
        ctx.accounts.minting_account.og_root = og_root_hash;
        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn update_wl_root(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        wl_list: String,
        wl_root_url: String,
        wl_root_hash: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.minting_account.wl_list_url = wl_list;
        ctx.accounts.minting_account.wl_root_url = wl_root_url;
        ctx.accounts.minting_account.wl_root = wl_root_hash;
        Ok(())
    }

    pub fn is_og_list(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        proof: Vec<[u8; 32]>,
    ) -> Result<()> {
        if proof.is_empty() {
            return Err(WalletMintErrors::InvalidProof.into());
        }
        let new_account = &ctx.accounts.admin;
        let node =
            anchor_lang::solana_program::keccak::hashv(&[&new_account.key.to_string().as_ref()]);
        if merkle_proof::verify(proof, ctx.accounts.minting_account.og_root, node.to_bytes())
            == false
        {
            return Err(WalletMintErrors::InvalidProof.into());
        }

        Ok(())
    }

    pub fn is_wl_list(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        proof: Vec<[u8; 32]>,
    ) -> Result<()> {
        if proof.is_empty() {
            return Err(WalletMintErrors::InvalidProof.into());
        }
        let new_account = &ctx.accounts.admin;
        let node =
            anchor_lang::solana_program::keccak::hashv(&[&new_account.key.to_string().as_ref()]);
        if merkle_proof::verify(proof, ctx.accounts.minting_account.wl_root, node.to_bytes())
            == false
        {
            return Err(WalletMintErrors::InvalidProof.into());
        }

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn update_price(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        new_og_price: u64,
        new_wl_price: u64,
        new_public_price: u64,
    ) -> Result<()> {
        if new_og_price > 0 {
            ctx.accounts.minting_account.og_price = new_og_price;
        }
        if new_wl_price > 0 {
            ctx.accounts.minting_account.wl_price = new_wl_price;
        }
        if new_public_price > 0 {
            ctx.accounts.minting_account.public_price = new_public_price;
        }

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn update_amount(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        new_og_amout: u64,
        new_wl_amout: u64,
        new_public_amout: u64,
    ) -> Result<()> {
        if new_og_amout > 0 {
            ctx.accounts.minting_account.og_max = new_og_amout;
        }
        if new_wl_amout > 0 {
            ctx.accounts.minting_account.wl_max = new_wl_amout;
        }
        if new_public_amout > 0 {
            ctx.accounts.minting_account.public_max = new_public_amout;
        }

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn set_stage(ctx: Context<CommonSt>, _nonce_minting: u8, new_stage: u8) -> Result<()> {
        if new_stage < 4 {
            ctx.accounts.minting_account.cur_stage = new_stage;
        }
        // 1 => OG/WL; 2 => (not used); 3 => Public;
        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn set_uri(ctx: Context<CommonSt>, _nonce_minting: u8, new_uri: String) -> Result<()> {
        ctx.accounts.minting_account.base_uri = new_uri;
        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn set_title(ctx: Context<CommonSt>, _nonce_minting: u8, new_title: String) -> Result<()> {
        ctx.accounts.minting_account.base_title = new_title;
        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn set_symbol(
        ctx: Context<CommonSt>,
        _nonce_minting: u8,
        new_symbol: String,
    ) -> Result<()> {
        ctx.accounts.minting_account.symbol = new_symbol;
        Ok(())
    }

    pub fn mint_nft_wl(ctx: Context<MintNFT>, proof: Vec<[u8; 32]>) -> Result<()> {
        if proof.is_empty() {
            return Err(WalletMintErrors::InvalidProof.into());
        }

        if ctx.accounts.minting_account.cur_stage == 0 {
            return Err(WalletMintErrors::MintDisabled.into());
        }

        if ctx.accounts.minting_account.cur_stage != 1
            && ctx.accounts.minting_account.cur_stage != 2
        {
            return Err(WalletMintErrors::NotWhitelistStage.into());
        }
        // set user minting info
        let mut _max_num = 0;
        let mut _price = 0;
        let mut _state = 0;

        let new_account = &ctx.accounts.mint_authority;
        let node =
            anchor_lang::solana_program::keccak::hashv(&[&new_account.key.to_string().as_ref()]);
        if merkle_proof::verify(
            proof.clone(),
            ctx.accounts.minting_account.og_root,
            node.to_bytes(),
        ) {
            _max_num = ctx.accounts.minting_account.og_max;
            _price = ctx.accounts.minting_account.og_price;
            _state = 1;
        }
        if merkle_proof::verify(
            proof.clone(),
            ctx.accounts.minting_account.wl_root,
            node.to_bytes(),
        ) {
            _max_num = ctx.accounts.minting_account.wl_max;
            _price = ctx.accounts.minting_account.wl_price;
            _state = 1;
        }
        if _max_num == 0 || _price == 0 || _state == 0 {
            return Err(WalletMintErrors::NotWhitelisted.into());
        }
        if ctx.accounts.user_minting_counter_account.cur_num_whitelist >= _max_num {
            return Err(WalletMintErrors::MaxWhitelistSupplyReached.into());
        }

        return _mint_nft(ctx, _price, _state, true);
    }

    pub fn mint_nft(ctx: Context<MintNFT>) -> Result<()> {
        // set user minting info
        let mut _max_num = ctx.accounts.minting_account.public_max;
        let mut _price = ctx.accounts.minting_account.public_price;
        let mut _state: u8 = 3;

        if ctx.accounts.minting_account.cur_stage == 0 {
            return Err(WalletMintErrors::MintDisabled.into());
        }

        if ctx.accounts.minting_account.cur_stage != 3 {
            return Err(WalletMintErrors::NotPublicStage.into());
        }

        if ctx.accounts.user_minting_counter_account.cur_num_public >= _max_num {
            return Err(WalletMintErrors::MaxPublicSupplyReached.into());
        }
        return _mint_nft(ctx, _price, _state, false);
    }

    #[access_control(is_admin(&ctx.accounts.minting_account, &ctx.accounts.admin))]
    pub fn mint_collection_nft(ctx: Context<MintCollectionNFT>) -> Result<()> {
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
            ctx.accounts.admin.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Account Info Assigned");
        let creator = vec![mpl_token_metadata::state::Creator {
            address: ctx.accounts.admin.key(),
            verified: false,
            share: 100,
        }];

        msg!("Creator Assigned");
        invoke(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.admin.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                "Degen Sweepers".to_string(),
                "DS".to_string(),
                "".to_string(),
                Some(creator),
                0,
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
            ctx.accounts.admin.to_account_info(),
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
                ctx.accounts.admin.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.payer.key(),
                Some(0),
            ),
            master_edition_infos.as_slice(),
        )?;
        msg!("Master Edition Nft Minted !!!");
        Ok(())
    }
}

fn _mint_nft(ctx: Context<MintNFT>, price: u64, state: u8, wl_mint: bool) -> Result<()> {
    if ctx.accounts.minting_account.max_supply <= ctx.accounts.minting_account.cur_num {
        return Err(WalletMintErrors::SoldOut.into());
    }
    if ctx.accounts.minting_account.cur_stage != state {
        return Err(WalletMintErrors::InvalidStage.into());
    }

    if ctx.accounts.minting_account.admin_key != *ctx.accounts.owner.key {
        return Err(WalletMintErrors::InvalidOwner.into());
    }

    let transfer_sol_to_seller =
        system_instruction::transfer(ctx.accounts.payer.key, ctx.accounts.owner.key, price);

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

    let maker = &ctx.accounts.maker;

    let account_info = vec![
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
        maker.to_account_info(),
    ];
    msg!("Account Info Assigned");
    let creator = vec![
        mpl_token_metadata::state::Creator {
            address: maker.key(),
            verified: true,
            share: 100,
        },
        mpl_token_metadata::state::Creator {
            address: ctx.accounts.mint_authority.key(),
            verified: false,
            share: 0,
        },
    ];
    let new_uri = format!(
        "{}{}{}",
        ctx.accounts.minting_account.base_uri, ctx.accounts.minting_account.cur_num, ".json"
    );
    let name = format!(
        "{} #{}",
        ctx.accounts.minting_account.base_title, ctx.accounts.minting_account.cur_num
    );

    // let collection = Collection {
    //     key: ctx.accounts.collection.key(),
    //     verified: false,
    // };

    msg!("Creator Assigned");

    let (_creator, creator_bump) =
        Pubkey::find_program_address(&[constants::NFT_CREATOR_SEED.as_bytes()], ctx.program_id);
    let authority_seeds = [constants::NFT_CREATOR_SEED.as_bytes(), &[creator_bump]];

    invoke_signed(
        &create_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.payer.key(),
            maker.key(),
            name,
            ctx.accounts.minting_account.symbol.to_string(),
            new_uri,
            Some(creator),
            1000,
            true,
            false,
            None, // Some(collection),
            None,
        ),
        account_info.as_slice(),
        &[&authority_seeds],
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
        maker.to_account_info(),
    ];
    msg!("Master Edition Account Infos Assigned");
    invoke_signed(
        &create_master_edition_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.master_edition.key(),
            ctx.accounts.mint.key(),
            maker.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.payer.key(),
            Some(0),
        ),
        master_edition_infos.as_slice(),
        &[&authority_seeds],
    )?;
    msg!("Master Edition Nft Minted !!!");

    // msg!("Set and verify collection");
    // invoke(
    //     &verify_collection(
    //         ctx.accounts.token_metadata_program.key(),
    //         ctx.accounts.metadata.key(),
    //         ctx.accounts.owner.key(),
    //         ctx.accounts.payer.key(),
    //         ctx.accounts.collection.key(),
    //         ctx.accounts.collection.key(),
    //         ctx.accounts.collection.key(),
    //         Some(ctx.accounts.collection.key()),
    //     ),
    //     master_edition_infos.as_slice(),
    // )?;

    if wl_mint {
        ctx.accounts.user_minting_counter_account.cur_num_whitelist += 1;
    } else {
        ctx.accounts.user_minting_counter_account.cur_num_public += 1;
    }
    ctx.accounts.minting_account.cur_num += 1;
    Ok(())
}

fn is_admin<'info>(
    minting_account: &Account<'info, MintingAccount>,
    signer: &Signer<'info>,
) -> Result<()> {
    if minting_account.admin_key != *signer.key {
        return Err(WalletMintErrors::Unauthorized.into());
    }
    Ok(())
}

#[derive(Accounts)]
#[instruction(_nonce_minting: u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = initializer,
        seeds = [ constants::MINTING_PDA_SEED.as_ref() ],
        bump,
        space = 8 + size_of::<MintingAccount>()
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
    pub max_supply: u64,
    pub og_max: u64,
    pub wl_max: u64,
    pub public_max: u64,
    pub og_price: u64,
    pub wl_price: u64,
    pub public_price: u64,
    pub og_root: [u8; 32],
    pub wl_root: [u8; 32],
    pub og_list_url: String,
    pub og_root_url: String,
    pub wl_list_url: String,
    pub wl_root_url: String,
    pub cur_num: u64,
    pub cur_stage: u8,
    pub base_uri: String,
    pub base_title: String,
    pub symbol: String,
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
    pub cur_num_public: u64,
    pub cur_num_whitelist: u64,
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
        space = 8 + size_of::<UserMintingAccount>()
    )]
    pub user_minting_counter_account: Box<Account<'info, UserMintingAccount>>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    // pub collection: AccountInfo<'info>,
    /// CHECK: account constraints checked in account trait
    #[account(mut)]
    pub maker: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct MintCollectionNFT<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
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
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
}

#[error_code]
pub enum WalletMintErrors {
    #[msg("Unauthorized.")]
    Unauthorized,
    #[msg("Invalid Merkle proof.")]
    InvalidProof,
    #[msg("Mint disabled.")]
    MintDisabled,
    #[msg("Not whitelist stage.")]
    NotWhitelistStage,
    #[msg("Not public stage.")]
    NotPublicStage,
    #[msg("Not whitelisted.")]
    NotWhitelisted,
    #[msg("Sold out.")]
    SoldOut,
    #[msg("Max whitelist supply reached.")]
    MaxWhitelistSupplyReached,
    #[msg("Max public supply reached.")]
    MaxPublicSupplyReached,
    #[msg("Invalid stage.")]
    InvalidStage,
    #[msg("Invalid owner.")]
    InvalidOwner,
}
