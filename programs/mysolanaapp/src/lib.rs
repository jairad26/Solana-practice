use anchor_lang::prelude::*;

declare_id!("6gMjtJLGGww8RkZHGWpnxa6ygitob7uYFod5AKEdfv8N");

#[program]
mod mysolanaapp {
    use super::*;
    //create and increment are RPC req handlers to call from a client app to interact w program
    //first parameter of any RPC handler is Context structure
    pub fn create(ctx: Context<Create>) -> Result<()> { //variable ctx is of type Context<Create>. Create is a struct declared later
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> { //variable ctx is of type Context<Increment>. Increment is a struct declared later
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> { //struct with parameters base_acct, user, system_program
    #[account(init, payer = user, space = 16 + 16)] //defines constraints and instructions related to proceeding account
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// An account that goes inside a transaction instruction
#[account]
pub struct BaseAccount {
    pub count: u64,
}