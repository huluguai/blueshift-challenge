use anchor_lang::prelude::*;

declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;
    use anchor_lang::system_program::{transfer, Transfer};
    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        // check if vault is empty
        require_eq!(ctx.accounts.vault.lamports(), 0, VaultError::VaultAlreadyExists);
        // Ensure amount exceeds rent-exempt minimum
        require!(amount > Rent::get()?.minimum_balance(0), VaultError::InvalidAmount);
        
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        // check if vault has any lamports
        require_neq!(ctx.accounts.vault.lamports(),0,VaultError::InvalidAmount);
        // create PDA account for the withdraw
        let signer_key = ctx.accounts.signer.key();
        let signer_seeds = &[b"vault",signer_key.as_ref(),&[ctx.bumps.vault]];
        transfer(
            //从 pda账户转账到signer账户，要指定pda账户的seeds，进行PDA验证，签名需要指定pda账户的seeds
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.signer.to_account_info(),
                },
                &[&signer_seeds[..]]
            ),
            ctx.accounts.vault.lamports(),
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault",signer.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    // 用于判断账户中是否已经有 lamports，因为这意味着金库已经存在
    #[msg("Vault already exists")]
    VaultAlreadyExists,
    // 我们不能存入少于基本账户最低租金的金额，因此我们检查金额是否大于该值
    #[msg("Invalid amount")]
    InvalidAmount,
}