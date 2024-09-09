use anchor_lang::{prelude::*, solana_program::message};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

declare_id!("HStEahcW144rLRp77a7eK48RGZRzRJabFMkWY7VyU5Aw");

#[program]
pub mod event_ticketing_app {
    use super::*;

    pub fn create_event(
        ctx: Context<CreateEvent>,
        name: String,
        ticket_number: u64,
        ticket_price: u64,
        
    ) -> Result<()> {

        ctx.accounts.event_account.set_inner(EventAccount{
            owner:ctx.accounts.owner.key(),
            mint_token:ctx.accounts.mint_token.key(),
            event_token_account:ctx.accounts.event_token_account.key(),
            name: name.clone(),
            ticket_number: ticket_number,
            ticket_price: ticket_price,
            bump: ctx.bumps.event_account,
        });

        msg!("Event is created!");

        Ok(())
    }

    pub fn ticket_purchase(ctx: Context<TicketPurchase>, price: u64) -> Result<()> {
        let hesap = &mut ctx.accounts.event_account;
        
        let owner_ata = ctx.accounts.owner_token_account.to_account_info();
        let event_ata = ctx.accounts.event_token_account.to_account_info();

        //cpi call
        let hesaplar = Transfer{
            from:owner_ata,
            to:event_ata,
            authority:ctx.accounts.owner.to_account_info()
        };

        let paralel_calistirma = 
        CpiContext::new(ctx.accounts.token_program.to_account_info(), hesaplar);

        hesap.ticket_price = price;

        transfer(paralel_calistirma, price);
        Ok(())
    }

    pub fn ticket_transfer(ctx: Context<TicketTransfer>, price: u64) -> Result<()> {
        /* let hesap = &mut ctx.accounts.event_account;
        
        let owner_ata = ctx.accounts.owner_token_account.to_account_info();
        let event_ata = ctx.accounts.event_token_account.to_account_info();

        //cpi call
        let hesaplar = Transfer{
            from:owner_ata,
            to:event_ata,
            authority:ctx.accounts.owner.to_account_info()
        };

        let seeds = &[
            &b"event"[..],
            &ctx.accounts.owner.key().to_bytes(),
            &ctx.accounts.token.key().to_bytes(),
            &[hesap.bump],
        ];

        let signerseed = &[&seeds[..]];

        let paralel_calistirma = 
        CpiContext::new(ctx.accounts.token_program.to_account_info(), hesaplar);

        hesap.ticket_price = price;

        transfer(paralel_calistirma, price); */

        Ok(())
    }

    pub fn delete_event(ctx: Context<DeleteEvent>) -> Result<()>{
        let hesap = &mut ctx.accounts.event_account;
        let owner = &ctx.accounts.owner;
        hesap.close(owner.to_account_info())
    }
}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(
        init, 
        payer = owner, 
        space = 8 + 32 + 32 + 32 + (4 + 20) + 8 + 8 + 1,
        seeds = [b"event", owner.key.as_ref(), mint_token.key().as_ref()], 
        bump,
    )]
    pub event_account: Account<'info, EventAccount>,

    #[account(
        init,
        payer = owner,
        associated_token::mint = mint_token,
        associated_token::authority = event_account
    )]
    pub event_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint_token: Account<'info, Mint>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TicketPurchase<'info>{
    #[account(
        mut,
        seeds = [b"event", owner.key.as_ref(), mint_token.key().as_ref()], 
        bump = event_account.bump
    )]
    pub event_account: Account<'info, EventAccount>,

    #[account(
        mut,
        associated_token::mint = mint_token,
        associated_token::authority = event_account
    )]
    pub event_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_token,
        associated_token::authority = owner
    )]
    pub owner_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint_token: Account<'info, Mint>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TicketTransfer<'info>{
    #[account(
        mut,
        seeds = [b"event", owner.key.as_ref(), mint_token.key().as_ref()], 
        bump = event_account.bump
    )]
    pub event_account: Account<'info, EventAccount>,

    #[account(
        mut,
        associated_token::mint = mint_token,
        associated_token::authority = event_account
    )]
    pub event_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_token,
        associated_token::authority = owner
    )]
    pub owner_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint_token: Account<'info, Mint>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct DeleteEvent<'info> {
    #[account(
        mut,
        seeds = [b"event", owner.key.as_ref(), mint_token.key().as_ref()], 
        bump = event_account.bump
    )]
    pub event_account: Account<'info, EventAccount>,

    #[account(mut)]
    pub mint_token: Account<'info, Mint>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EventAccount {
    pub owner: Pubkey,
    pub mint_token: Pubkey,
    pub event_token_account: Pubkey,
    pub name: String,
    pub ticket_number: u64,
    pub ticket_price: u64,
    pub bump: u8,
}

