use anchor_lang::prelude::*;
use crate::state::{
    acl::{self, *},
    StateAccount,
};
use drift::program::Drift;
use drift::typedefs::*;
#[derive(Accounts)]
pub struct DriftInitializeUser<'info> {
    pub glam_state: Box<Account<'info, StateAccount>>,
    #[account(
        seeds = [crate::constants::SEED_VAULT.as_bytes(),
        glam_state.key().as_ref()],
        bump
    )]
    pub glam_vault: SystemAccount<'info>,
    #[account(mut)]
    pub glam_signer: Signer<'info>,
    pub cpi_program: Program<'info, Drift>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub user: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub user_stats: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub state: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(address = glam_state.vault)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct DriftInitializeUserStats<'info> {
    pub glam_state: Box<Account<'info, StateAccount>>,
    #[account(
        seeds = [crate::constants::SEED_VAULT.as_bytes(),
        glam_state.key().as_ref()],
        bump
    )]
    pub glam_vault: SystemAccount<'info>,
    #[account(mut)]
    pub glam_signer: Signer<'info>,
    pub cpi_program: Program<'info, Drift>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub user_stats: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub state: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(address = glam_state.vault)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct DriftDeposit<'info> {
    pub glam_state: Box<Account<'info, StateAccount>>,
    #[account(
        mut,
        seeds = [crate::constants::SEED_VAULT.as_bytes(),
        glam_state.key().as_ref()],
        bump
    )]
    pub glam_vault: SystemAccount<'info>,
    #[account(mut)]
    pub glam_signer: Signer<'info>,
    pub cpi_program: Program<'info, Drift>,
    /// CHECK: should be validated by target program
    pub state: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub user: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub user_stats: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(address = glam_state.vault)]
    pub authority: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub spot_market_vault: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    pub token_program: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct DriftWithdraw<'info> {
    pub glam_state: Box<Account<'info, StateAccount>>,
    #[account(
        mut,
        seeds = [crate::constants::SEED_VAULT.as_bytes(),
        glam_state.key().as_ref()],
        bump
    )]
    pub glam_vault: SystemAccount<'info>,
    #[account(mut)]
    pub glam_signer: Signer<'info>,
    pub cpi_program: Program<'info, Drift>,
    /// CHECK: should be validated by target program
    pub state: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub user: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub user_stats: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(address = glam_state.vault)]
    pub authority: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub spot_market_vault: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    pub drift_signer: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    pub token_program: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct DriftCancelOrders<'info> {
    pub glam_state: Box<Account<'info, StateAccount>>,
    #[account(
        seeds = [crate::constants::SEED_VAULT.as_bytes(),
        glam_state.key().as_ref()],
        bump
    )]
    pub glam_vault: SystemAccount<'info>,
    #[account(mut)]
    pub glam_signer: Signer<'info>,
    pub cpi_program: Program<'info, Drift>,
    /// CHECK: should be validated by target program
    pub state: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub user: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(address = glam_state.vault)]
    pub authority: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct DriftModifyOrder<'info> {
    pub glam_state: Box<Account<'info, StateAccount>>,
    #[account(
        seeds = [crate::constants::SEED_VAULT.as_bytes(),
        glam_state.key().as_ref()],
        bump
    )]
    pub glam_vault: SystemAccount<'info>,
    #[account(mut)]
    pub glam_signer: Signer<'info>,
    pub cpi_program: Program<'info, Drift>,
    /// CHECK: should be validated by target program
    pub state: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub user: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(address = glam_state.vault)]
    pub authority: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct DriftUpdateUser<'info> {
    pub glam_state: Box<Account<'info, StateAccount>>,
    #[account(
        seeds = [crate::constants::SEED_VAULT.as_bytes(),
        glam_state.key().as_ref()],
        bump
    )]
    pub glam_vault: SystemAccount<'info>,
    #[account(mut)]
    pub glam_signer: Signer<'info>,
    pub cpi_program: Program<'info, Drift>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub user: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(address = glam_state.vault)]
    pub authority: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct DriftDeleteUser<'info> {
    pub glam_state: Box<Account<'info, StateAccount>>,
    #[account(
        mut,
        seeds = [crate::constants::SEED_VAULT.as_bytes(),
        glam_state.key().as_ref()],
        bump
    )]
    pub glam_vault: SystemAccount<'info>,
    #[account(mut)]
    pub glam_signer: Signer<'info>,
    pub cpi_program: Program<'info, Drift>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub user: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub user_stats: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(mut)]
    pub state: AccountInfo<'info>,
    /// CHECK: should be validated by target program
    #[account(mut, address = glam_state.vault)]
    pub authority: AccountInfo<'info>,
}
#[access_control(
    acl::check_access(
        &ctx.accounts.glam_state,
        &ctx.accounts.glam_signer.key,
        Permission::DriftInitialize
    )
)]
#[access_control(acl::check_integration(&ctx.accounts.glam_state, Integration::Drift))]
#[glam_macros::glam_vault_signer_seeds]
pub fn drift_initialize_user(
    ctx: Context<DriftInitializeUser>,
    sub_account_id: u16,
    name: [u8; 32],
) -> Result<()> {
    drift::cpi::initialize_user(
        CpiContext::new_with_signer(
            ctx.accounts.cpi_program.to_account_info(),
            drift::cpi::accounts::InitializeUser {
                user: ctx.accounts.user.to_account_info(),
                user_stats: ctx.accounts.user_stats.to_account_info(),
                state: ctx.accounts.state.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            glam_vault_signer_seeds,
        ),
        sub_account_id,
        name,
    )
}
#[access_control(
    acl::check_access(
        &ctx.accounts.glam_state,
        &ctx.accounts.glam_signer.key,
        Permission::DriftInitialize
    )
)]
#[access_control(acl::check_integration(&ctx.accounts.glam_state, Integration::Drift))]
#[glam_macros::glam_vault_signer_seeds]
pub fn drift_initialize_user_stats(
    ctx: Context<DriftInitializeUserStats>,
) -> Result<()> {
    drift::cpi::initialize_user_stats(
        CpiContext::new_with_signer(
            ctx.accounts.cpi_program.to_account_info(),
            drift::cpi::accounts::InitializeUserStats {
                user_stats: ctx.accounts.user_stats.to_account_info(),
                state: ctx.accounts.state.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            glam_vault_signer_seeds,
        ),
    )
}
#[access_control(
    acl::check_access(
        &ctx.accounts.glam_state,
        &ctx.accounts.glam_signer.key,
        Permission::DriftDeposit
    )
)]
#[access_control(acl::check_integration(&ctx.accounts.glam_state, Integration::Drift))]
#[glam_macros::glam_vault_signer_seeds]
pub fn drift_deposit<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, DriftDeposit<'info>>,
    market_index: u16,
    amount: u64,
    reduce_only: bool,
) -> Result<()> {
    drift::cpi::deposit(
        CpiContext::new_with_signer(
                ctx.accounts.cpi_program.to_account_info(),
                drift::cpi::accounts::Deposit {
                    state: ctx.accounts.state.to_account_info(),
                    user: ctx.accounts.user.to_account_info(),
                    user_stats: ctx.accounts.user_stats.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                    spot_market_vault: ctx.accounts.spot_market_vault.to_account_info(),
                    user_token_account: ctx
                        .accounts
                        .user_token_account
                        .to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
                glam_vault_signer_seeds,
            )
            .with_remaining_accounts(ctx.remaining_accounts.to_vec()),
        market_index,
        amount,
        reduce_only,
    )
}
#[access_control(
    acl::check_access(
        &ctx.accounts.glam_state,
        &ctx.accounts.glam_signer.key,
        Permission::DriftWithdraw
    )
)]
#[access_control(acl::check_integration(&ctx.accounts.glam_state, Integration::Drift))]
#[glam_macros::glam_vault_signer_seeds]
pub fn drift_withdraw<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, DriftWithdraw<'info>>,
    market_index: u16,
    amount: u64,
    reduce_only: bool,
) -> Result<()> {
    drift::cpi::withdraw(
        CpiContext::new_with_signer(
                ctx.accounts.cpi_program.to_account_info(),
                drift::cpi::accounts::Withdraw {
                    state: ctx.accounts.state.to_account_info(),
                    user: ctx.accounts.user.to_account_info(),
                    user_stats: ctx.accounts.user_stats.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                    spot_market_vault: ctx.accounts.spot_market_vault.to_account_info(),
                    drift_signer: ctx.accounts.drift_signer.to_account_info(),
                    user_token_account: ctx
                        .accounts
                        .user_token_account
                        .to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
                glam_vault_signer_seeds,
            )
            .with_remaining_accounts(ctx.remaining_accounts.to_vec()),
        market_index,
        amount,
        reduce_only,
    )
}
#[access_control(
    acl::check_access(
        &ctx.accounts.glam_state,
        &ctx.accounts.glam_signer.key,
        Permission::DriftCancelOrders
    )
)]
#[access_control(acl::check_integration(&ctx.accounts.glam_state, Integration::Drift))]
#[glam_macros::glam_vault_signer_seeds]
pub fn drift_cancel_orders<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, DriftCancelOrders<'info>>,
    market_type: Option<MarketType>,
    market_index: Option<u16>,
    direction: Option<PositionDirection>,
) -> Result<()> {
    drift::cpi::cancel_orders(
        CpiContext::new_with_signer(
                ctx.accounts.cpi_program.to_account_info(),
                drift::cpi::accounts::CancelOrders {
                    state: ctx.accounts.state.to_account_info(),
                    user: ctx.accounts.user.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
                glam_vault_signer_seeds,
            )
            .with_remaining_accounts(ctx.remaining_accounts.to_vec()),
        market_type,
        market_index,
        direction,
    )
}
#[access_control(
    acl::check_access(
        &ctx.accounts.glam_state,
        &ctx.accounts.glam_signer.key,
        Permission::DriftCancelOrders
    )
)]
#[access_control(acl::check_integration(&ctx.accounts.glam_state, Integration::Drift))]
#[glam_macros::glam_vault_signer_seeds]
pub fn drift_cancel_orders_by_ids<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, DriftCancelOrders<'info>>,
    order_ids: Vec<u32>,
) -> Result<()> {
    drift::cpi::cancel_orders_by_ids(
        CpiContext::new_with_signer(
                ctx.accounts.cpi_program.to_account_info(),
                drift::cpi::accounts::CancelOrdersByIds {
                    state: ctx.accounts.state.to_account_info(),
                    user: ctx.accounts.user.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
                glam_vault_signer_seeds,
            )
            .with_remaining_accounts(ctx.remaining_accounts.to_vec()),
        order_ids,
    )
}
#[access_control(
    acl::check_access(
        &ctx.accounts.glam_state,
        &ctx.accounts.glam_signer.key,
        Permission::DriftModifyOrders
    )
)]
#[access_control(acl::check_integration(&ctx.accounts.glam_state, Integration::Drift))]
#[glam_macros::glam_vault_signer_seeds]
pub fn drift_modify_order(
    ctx: Context<DriftModifyOrder>,
    order_id: Option<u32>,
    modify_order_params: ModifyOrderParams,
) -> Result<()> {
    drift::cpi::modify_order(
        CpiContext::new_with_signer(
            ctx.accounts.cpi_program.to_account_info(),
            drift::cpi::accounts::ModifyOrder {
                state: ctx.accounts.state.to_account_info(),
                user: ctx.accounts.user.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
            glam_vault_signer_seeds,
        ),
        order_id,
        modify_order_params,
    )
}
#[access_control(
    acl::check_access(
        &ctx.accounts.glam_state,
        &ctx.accounts.glam_signer.key,
        Permission::DriftUpdateUser
    )
)]
#[access_control(acl::check_integration(&ctx.accounts.glam_state, Integration::Drift))]
#[glam_macros::glam_vault_signer_seeds]
pub fn drift_update_user_custom_margin_ratio(
    ctx: Context<DriftUpdateUser>,
    sub_account_id: u16,
    margin_ratio: u32,
) -> Result<()> {
    drift::cpi::update_user_custom_margin_ratio(
        CpiContext::new_with_signer(
            ctx.accounts.cpi_program.to_account_info(),
            drift::cpi::accounts::UpdateUserCustomMarginRatio {
                user: ctx.accounts.user.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
            glam_vault_signer_seeds,
        ),
        sub_account_id,
        margin_ratio,
    )
}
#[access_control(
    acl::check_access(
        &ctx.accounts.glam_state,
        &ctx.accounts.glam_signer.key,
        Permission::DriftUpdateUser
    )
)]
#[access_control(acl::check_integration(&ctx.accounts.glam_state, Integration::Drift))]
#[glam_macros::glam_vault_signer_seeds]
pub fn drift_update_user_margin_trading_enabled(
    ctx: Context<DriftUpdateUser>,
    sub_account_id: u16,
    margin_trading_enabled: bool,
) -> Result<()> {
    drift::cpi::update_user_margin_trading_enabled(
        CpiContext::new_with_signer(
            ctx.accounts.cpi_program.to_account_info(),
            drift::cpi::accounts::UpdateUserMarginTradingEnabled {
                user: ctx.accounts.user.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
            glam_vault_signer_seeds,
        ),
        sub_account_id,
        margin_trading_enabled,
    )
}
#[access_control(
    acl::check_access(
        &ctx.accounts.glam_state,
        &ctx.accounts.glam_signer.key,
        Permission::DriftUpdateUser
    )
)]
#[access_control(acl::check_integration(&ctx.accounts.glam_state, Integration::Drift))]
#[glam_macros::glam_vault_signer_seeds]
pub fn drift_update_user_delegate(
    ctx: Context<DriftUpdateUser>,
    sub_account_id: u16,
    delegate: Pubkey,
) -> Result<()> {
    drift::cpi::update_user_delegate(
        CpiContext::new_with_signer(
            ctx.accounts.cpi_program.to_account_info(),
            drift::cpi::accounts::UpdateUserDelegate {
                user: ctx.accounts.user.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
            glam_vault_signer_seeds,
        ),
        sub_account_id,
        delegate,
    )
}
#[access_control(
    acl::check_access(
        &ctx.accounts.glam_state,
        &ctx.accounts.glam_signer.key,
        Permission::DriftDeleteUser
    )
)]
#[access_control(acl::check_integration(&ctx.accounts.glam_state, Integration::Drift))]
#[glam_macros::glam_vault_signer_seeds]
pub fn drift_delete_user(ctx: Context<DriftDeleteUser>) -> Result<()> {
    drift::cpi::delete_user(
        CpiContext::new_with_signer(
            ctx.accounts.cpi_program.to_account_info(),
            drift::cpi::accounts::DeleteUser {
                user: ctx.accounts.user.to_account_info(),
                user_stats: ctx.accounts.user_stats.to_account_info(),
                state: ctx.accounts.state.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
            glam_vault_signer_seeds,
        ),
    )
}

