use anchor_lang::prelude::*;
use anchor_lang::solana_program::{clock};
// use anchor_spl::token::{TokenAccount, Token, Mint};
use std::convert::Into;
use std::convert::TryInto;
use sha2::{Sha256, Digest};

declare_id!("3bPsib4ztst1XsseFxKdqxVPitsfzZ5RGY9E9ATLZXnj");
mod constants {
    pub const MIN_DEPOSIT_AMOUNT: u64 = 1_000;
}

enum EvenOrOdd {
    Even,
    Odd
}

impl From<usize> for EvenOrOdd {
    fn from(x: usize) -> EvenOrOdd {
        if x % 2 == 0 {
            EvenOrOdd::Even
        } else {
            EvenOrOdd::Odd
        }
    }
}

#[program]
pub mod deposit_game {
    use super::*;
    pub fn initialize(
                ctx: Context<Initialize>, 
                nonce: u8,
                ) -> ProgramResult {
        let pool = &mut ctx.accounts.pool;
        pool.authority = ctx.accounts.authority.key();
        pool.nonce = nonce;
        pool.game_count = 0;
        pool.game5_count = 0;
        pool.game10_count = 0;
        pool.game_finished = "0".to_string();
        pool.game5_finished = "0".to_string();
        pool.game10_finished = "0".to_string();
        
        Ok(())
    }

    pub fn create_game(ctx: Context<CreateGame>, nonce: u8, vault_nonce: u8, id: String) -> ProgramResult {

        let game = &mut ctx.accounts.game;
        game.authority = ctx.accounts.pool.authority;

        game.account1 = ctx.accounts.vault.key();
        game.account2 = ctx.accounts.vault.key();
        game.account3 = ctx.accounts.vault.key();
        game.account_deposited1 = false;
        game.account_deposited2 = false;
        game.account_deposited3 = false;
        game.vault = ctx.accounts.vault.key();
        game.creator = ctx.accounts.signer.key();
        game.nonce = nonce;
        game.vault_nonce = vault_nonce;
        game.id = id;
        
        let pool = &mut ctx.accounts.pool;
        pool.game_count = pool.game_count.checked_add(1).unwrap();

        Ok(())
    }

    pub fn create_game5(ctx: Context<CreateGame5>, nonce: u8, vault_nonce: u8, id: String) -> ProgramResult {
        
        let game = &mut ctx.accounts.game;
        game.authority = ctx.accounts.pool.authority;

        game.account1 = ctx.accounts.vault.key();
        game.account2 = ctx.accounts.vault.key();
        game.account3 = ctx.accounts.vault.key();
        game.account4 = ctx.accounts.vault.key();
        game.account5 = ctx.accounts.vault.key();
        game.account_deposited1 = false;
        game.account_deposited2 = false;
        game.account_deposited3 = false;
        game.account_deposited4 = false;
        game.account_deposited5 = false;
        game.vault = ctx.accounts.vault.key();
        game.creator = ctx.accounts.signer.key();
        game.nonce = nonce;
        game.vault_nonce = vault_nonce;
        game.id = id;

        let pool = &mut ctx.accounts.pool;
        pool.game5_count = pool.game5_count.checked_add(1).unwrap();

        Ok(())
    }

    pub fn create_game10(ctx: Context<CreateGame10>, nonce: u8, vault_nonce: u8, id: String) -> ProgramResult {
        
        let game = &mut ctx.accounts.game;
        game.authority = ctx.accounts.pool.authority;

        game.account1 = ctx.accounts.vault.key();
        game.account2 = ctx.accounts.vault.key();
        game.account3 = ctx.accounts.vault.key();
        game.account4 = ctx.accounts.vault.key();
        game.account5 = ctx.accounts.vault.key();
        game.account6 = ctx.accounts.vault.key();
        game.account7 = ctx.accounts.vault.key();
        game.account8 = ctx.accounts.vault.key();
        game.account9 = ctx.accounts.vault.key();
        game.account10 = ctx.accounts.vault.key();
        game.account_deposited1 = false;
        game.account_deposited2 = false;
        game.account_deposited3 = false;
        game.account_deposited4 = false;
        game.account_deposited5 = false;
        game.account_deposited6 = false;
        game.account_deposited7 = false;
        game.account_deposited8 = false;
        game.account_deposited9 = false;
        game.account_deposited10 = false;
        game.vault = ctx.accounts.vault.key();
        game.creator = ctx.accounts.signer.key();
        game.nonce = nonce;
        game.vault_nonce = vault_nonce;
        game.id = id;
        
        let pool = &mut ctx.accounts.pool;
        pool.game10_count = pool.game10_count.checked_add(1).unwrap();

        Ok(())
    }

    pub fn create_odd_game(ctx: Context<CreateOddGame>, nonce: u8, vault_nonce: u8, id: String, odd: u8, players: u8, bid: u64) -> ProgramResult {
        
        let game = &mut ctx.accounts.game;
        game.authority = ctx.accounts.pool.authority;
        game.finished = false;
        game.deposited = 0;
        game.odd = odd;
        game.players = players;
        game.bid = bid;
        game.vault = ctx.accounts.vault.key();
        game.winners = [game.vault; 50];
        game.creator = ctx.accounts.signer.key();
        game.nonce = nonce;
        game.vault_nonce = vault_nonce;
        game.id = id;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
        if amount < constants::MIN_DEPOSIT_AMOUNT {
            return Err(ErrorCode::MinDepositAmount.into());
        }

        if ctx.accounts.game.account1 == ctx.accounts.depositor.key() 
            || ctx.accounts.game.account2 == ctx.accounts.depositor.key() 
            || ctx.accounts.game.account3 == ctx.accounts.depositor.key() {
            return Err(ErrorCode::AlreadyDeposit.into());
        }

        let ix = anchor_lang::solana_program::system_instruction::transfer(
                                    &ctx.accounts.depositor.key(), 
                                    &ctx.accounts.vault.key(), 
                                    amount);
        anchor_lang::solana_program::program::invoke(&ix, &[
                                                                ctx.accounts.depositor.to_account_info(), 
                                                                ctx.accounts.vault.to_account_info(), 
                                                            ])?;

        let current_time = clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();
        if ctx.accounts.game.account_deposited1 != true {
            ctx.accounts.game.account_deposited1 = true;
            ctx.accounts.game.account1 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date1 = current_time;
            ctx.accounts.game.deposit_amount1 = amount;
        } else if ctx.accounts.game.account_deposited2 != true {
            ctx.accounts.game.account_deposited2 = true;
            ctx.accounts.game.account2 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date2 = current_time;
            ctx.accounts.game.deposit_amount2 = amount;
        } else {
            ctx.accounts.game.account_deposited3 = true;
            ctx.accounts.game.account3 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date3 = current_time;
            ctx.accounts.game.deposit_amount3 = amount;
        }

        if ctx.accounts.game.account_deposited1 == true 
            && ctx.accounts.game.account_deposited2 == true
            && ctx.accounts.game.account_deposited3 == true {
                msg!("Passed all!");
                let seeds = &[
                    ctx.accounts.pool.to_account_info().key.as_ref(),
                    "vault".as_bytes(),
                    ctx.accounts.game.id.as_bytes(),
                    &[ctx.accounts.game.vault_nonce],
                ];
                let game_signer = &[&seeds[..]];
                let lamports = ctx.accounts.vault.to_account_info().lamports();
                // let e_or_o = EvenOrOdd::from(current_time as usize);
                let mut hasher = Sha256::new();
                hasher.update(ctx.accounts.game.account1.to_string() +
                            &ctx.accounts.game.deposit_date1.to_string() +
                            &ctx.accounts.game.account2.to_string() +
                            &ctx.accounts.game.deposit_date2.to_string()+
                            &ctx.accounts.game.account3.to_string()+
                            &ctx.accounts.game.deposit_date3.to_string()
                        );
                let digest = hasher.finalize();

                let string_flag = digest[30].checked_div(16).unwrap();
                if string_flag > 9 {
                    anchor_lang::solana_program::program::invoke_signed(
                        &anchor_lang::solana_program::system_instruction::transfer(
                            &ctx.accounts.vault.key(), 
                            &ctx.accounts.game.account3.key(), 
                            lamports
                        ),
                        &[
                            ctx.accounts.vault.to_account_info(),
                            ctx.accounts.depositor.to_account_info(),
                        ],
                        game_signer,
                    )?;
                } else {
                    let e_or_o = EvenOrOdd::from(string_flag as usize);
                    match e_or_o {
                        EvenOrOdd::Even => {
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::transfer(
                                    &ctx.accounts.vault.key(), 
                                    &ctx.accounts.game.account1.key(), 
                                    lamports
                                ),
                                &[
                                    ctx.accounts.vault.to_account_info(),
                                    ctx.accounts.account1.to_account_info(),
                                ],
                                game_signer,
                            )?;
                        }
                        EvenOrOdd::Odd => {
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::transfer(
                                    &ctx.accounts.vault.key(), 
                                    &ctx.accounts.game.account2.key(), 
                                    lamports
                                ),
                                &[
                                    ctx.accounts.vault.to_account_info(),
                                    ctx.accounts.account2.to_account_info(),
                                ],
                                game_signer,
                            )?;
                        }
                    }
                }

                // ctx.accounts.game.account1 = ctx.accounts.game.vault;
                // ctx.accounts.game.account2 = ctx.accounts.game.vault;
                // ctx.accounts.game.account3 = ctx.accounts.game.vault;
                // ctx.accounts.game.account_deposited1 = false;
                // ctx.accounts.game.account_deposited2 = false;
                // ctx.accounts.game.account_deposited3 = false;

                ctx.accounts.game.last_hash = digest.as_slice().try_into().expect("Wrong length");

                ctx.accounts.pool.game_finished = ctx.accounts.game.id.clone();
            }

        Ok(())
    }

    pub fn deposit5(ctx: Context<Deposit5>, amount: u64) -> ProgramResult {
        if amount < constants::MIN_DEPOSIT_AMOUNT {
            return Err(ErrorCode::MinDepositAmount.into());
        }

        if ctx.accounts.game.account1 == ctx.accounts.depositor.key() 
            || ctx.accounts.game.account2 == ctx.accounts.depositor.key() 
            || ctx.accounts.game.account3 == ctx.accounts.depositor.key() 
            || ctx.accounts.game.account4 == ctx.accounts.depositor.key() 
            || ctx.accounts.game.account5 == ctx.accounts.depositor.key() {
            return Err(ErrorCode::AlreadyDeposit.into());
        }

        let ix = anchor_lang::solana_program::system_instruction::transfer(
                                    &ctx.accounts.depositor.key(), 
                                    &ctx.accounts.vault.key(), 
                                    amount);
        anchor_lang::solana_program::program::invoke(&ix, &[
                                                                ctx.accounts.depositor.to_account_info(), 
                                                                ctx.accounts.vault.to_account_info(), 
                                                            ])?;

        let current_time = clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();
        if ctx.accounts.game.account_deposited1 != true {
            ctx.accounts.game.account_deposited1 = true;
            ctx.accounts.game.account1 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date1 = current_time;
            ctx.accounts.game.deposit_amount1 = amount;
        } else if ctx.accounts.game.account_deposited2 != true {
            ctx.accounts.game.account_deposited2 = true;
            ctx.accounts.game.account2 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date2 = current_time;
            ctx.accounts.game.deposit_amount2 = amount;
        } else if ctx.accounts.game.account_deposited3 != true {
            ctx.accounts.game.account_deposited3 = true;
            ctx.accounts.game.account3 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date3 = current_time;
            ctx.accounts.game.deposit_amount3 = amount;
        } else if ctx.accounts.game.account_deposited4 != true {
            ctx.accounts.game.account_deposited4 = true;
            ctx.accounts.game.account4 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date4 = current_time;
            ctx.accounts.game.deposit_amount4 = amount;
        } else {
            ctx.accounts.game.account_deposited5 = true;
            ctx.accounts.game.account5 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date5 = current_time;
            ctx.accounts.game.deposit_amount5 = amount;
        }

        if ctx.accounts.game.account_deposited1 == true 
            && ctx.accounts.game.account_deposited2 == true
            && ctx.accounts.game.account_deposited3 == true
            && ctx.accounts.game.account_deposited4 == true
            && ctx.accounts.game.account_deposited5 == true {
                msg!("Passed all!");
                let seeds = &[
                    ctx.accounts.pool.to_account_info().key.as_ref(),
                    "vault5".as_bytes(),
                    ctx.accounts.game.id.as_bytes(),
                    &[ctx.accounts.game.vault_nonce],
                ];
                let game_signer = &[&seeds[..]];
                let lamports = ctx.accounts.vault.to_account_info().lamports();
                let mut hasher = Sha256::new();
                hasher.update(
                            ctx.accounts.game.account1.to_string() +
                            &ctx.accounts.game.deposit_date1.to_string() +
                            &ctx.accounts.game.account2.to_string() +
                            &ctx.accounts.game.deposit_date2.to_string() +
                            &ctx.accounts.game.account3.to_string() +
                            &ctx.accounts.game.deposit_date3.to_string() +
                            &ctx.accounts.game.account4.to_string() +
                            &ctx.accounts.game.deposit_date4.to_string() +
                            &ctx.accounts.game.account5.to_string() +
                            &ctx.accounts.game.deposit_date5.to_string()
                        );
                let digest = hasher.finalize();

                let string_flag = digest[30].checked_div(16).unwrap();
                if string_flag > 9 {
                    msg!("Passed letter!");
                    anchor_lang::solana_program::program::invoke_signed(
                        &anchor_lang::solana_program::system_instruction::transfer(
                            &ctx.accounts.vault.key(), 
                            &ctx.accounts.game.account5.key(), 
                            lamports
                        ),
                        &[
                            ctx.accounts.vault.to_account_info(),
                            ctx.accounts.depositor.to_account_info(),
                        ],
                        game_signer,
                    )?;
                } else {
                    let e_or_o = EvenOrOdd::from(string_flag as usize);
                    match e_or_o {
                        EvenOrOdd::Even => {
                            msg!("Passed Even!");
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::transfer(
                                    &ctx.accounts.vault.key(), 
                                    &ctx.accounts.game.account1.key(), 
                                    lamports
                                ),
                                &[
                                    ctx.accounts.vault.to_account_info(),
                                    ctx.accounts.account1.to_account_info(),
                                ],
                                game_signer,
                            )?;
                        }
                        EvenOrOdd::Odd => {
                            msg!("Passed Odd!");
                            anchor_lang::solana_program::program::invoke_signed(
                                &anchor_lang::solana_program::system_instruction::transfer(
                                    &ctx.accounts.vault.key(), 
                                    &ctx.accounts.game.account3.key(), 
                                    lamports
                                ),
                                &[
                                    ctx.accounts.vault.to_account_info(),
                                    ctx.accounts.account3.to_account_info(),
                                ],
                                game_signer,
                            )?;
                        }
                    }
                }

                // ctx.accounts.game.account1 = ctx.accounts.game.vault;
                // ctx.accounts.game.account2 = ctx.accounts.game.vault;
                // ctx.accounts.game.account3 = ctx.accounts.game.vault;
                // ctx.accounts.game.account4 = ctx.accounts.game.vault;
                // ctx.accounts.game.account5 = ctx.accounts.game.vault;
                // ctx.accounts.game.account_deposited1 = false;
                // ctx.accounts.game.account_deposited2 = false;
                // ctx.accounts.game.account_deposited3 = false;
                // ctx.accounts.game.account_deposited4 = false;
                // ctx.accounts.game.account_deposited5 = false;
                ctx.accounts.game.last_hash = digest.as_slice().try_into().expect("Wrong length");

                ctx.accounts.pool.game5_finished = ctx.accounts.game.id.clone();
            }

        Ok(())
    }

    pub fn deposit10(ctx: Context<Deposit10>, amount: u64) -> ProgramResult {
        if amount < constants::MIN_DEPOSIT_AMOUNT {
            return Err(ErrorCode::MinDepositAmount.into());
        }

        if ctx.accounts.game.account1 == ctx.accounts.depositor.key() 
            || ctx.accounts.game.account2 == ctx.accounts.depositor.key() 
            || ctx.accounts.game.account3 == ctx.accounts.depositor.key() 
            || ctx.accounts.game.account4 == ctx.accounts.depositor.key() 
            || ctx.accounts.game.account5 == ctx.accounts.depositor.key() 
            || ctx.accounts.game.account6 == ctx.accounts.depositor.key() 
            || ctx.accounts.game.account7 == ctx.accounts.depositor.key() 
            || ctx.accounts.game.account8 == ctx.accounts.depositor.key() 
            || ctx.accounts.game.account9 == ctx.accounts.depositor.key() 
            || ctx.accounts.game.account10 == ctx.accounts.depositor.key() {
            return Err(ErrorCode::AlreadyDeposit.into());
        }

        let ix = anchor_lang::solana_program::system_instruction::transfer(
                                    &ctx.accounts.depositor.key(), 
                                    &ctx.accounts.vault.key(), 
                                    amount);
        anchor_lang::solana_program::program::invoke(&ix, &[
                                                                ctx.accounts.depositor.to_account_info(), 
                                                                ctx.accounts.vault.to_account_info(), 
                                                            ])?;

        let current_time = clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();
        if ctx.accounts.game.account_deposited1 != true {
            ctx.accounts.game.account_deposited1 = true;
            ctx.accounts.game.account1 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date1 = current_time;
            ctx.accounts.game.deposit_amount1 = amount;
        } else if ctx.accounts.game.account_deposited2 != true {
            ctx.accounts.game.account_deposited2 = true;
            ctx.accounts.game.account2 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date2 = current_time;
            ctx.accounts.game.deposit_amount2 = amount;
        } else if ctx.accounts.game.account_deposited3 != true {
            ctx.accounts.game.account_deposited3 = true;
            ctx.accounts.game.account3 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date3 = current_time;
            ctx.accounts.game.deposit_amount3 = amount;
        } else if ctx.accounts.game.account_deposited4 != true {
            ctx.accounts.game.account_deposited4 = true;
            ctx.accounts.game.account4 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date4 = current_time;
            ctx.accounts.game.deposit_amount4 = amount;
        } else if ctx.accounts.game.account_deposited5 != true {
            ctx.accounts.game.account_deposited5 = true;
            ctx.accounts.game.account5 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date5 = current_time;
            ctx.accounts.game.deposit_amount5 = amount;
        } else if ctx.accounts.game.account_deposited6 != true {
            ctx.accounts.game.account_deposited6 = true;
            ctx.accounts.game.account6 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date6 = current_time;
            ctx.accounts.game.deposit_amount6 = amount;
        } else if ctx.accounts.game.account_deposited7 != true {
            ctx.accounts.game.account_deposited7 = true;
            ctx.accounts.game.account7 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date7 = current_time;
            ctx.accounts.game.deposit_amount7 = amount;
        } else if ctx.accounts.game.account_deposited8 != true {
            ctx.accounts.game.account_deposited8 = true;
            ctx.accounts.game.account8 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date8 = current_time;
            ctx.accounts.game.deposit_amount8 = amount;
        } else if ctx.accounts.game.account_deposited9 != true {
            ctx.accounts.game.account_deposited9 = true;
            ctx.accounts.game.account9 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date9 = current_time;
            ctx.accounts.game.deposit_amount9 = amount;
        } else {
            ctx.accounts.game.account_deposited10 = true;
            ctx.accounts.game.account10 = ctx.accounts.depositor.key();
            ctx.accounts.game.deposit_date10 = current_time;
            ctx.accounts.game.deposit_amount10 = amount;
        }

        if ctx.accounts.game.account_deposited1 == true 
            && ctx.accounts.game.account_deposited2 == true
            && ctx.accounts.game.account_deposited3 == true
            && ctx.accounts.game.account_deposited4 == true
            && ctx.accounts.game.account_deposited5 == true
            && ctx.accounts.game.account_deposited6 == true
            && ctx.accounts.game.account_deposited7 == true
            && ctx.accounts.game.account_deposited8 == true
            && ctx.accounts.game.account_deposited9 == true
            && ctx.accounts.game.account_deposited10 == true {
                msg!("Passed all!");
                let seeds = &[
                    ctx.accounts.pool.to_account_info().key.as_ref(),
                    "vault10".as_bytes(),
                    ctx.accounts.game.id.as_bytes(),
                    &[ctx.accounts.game.vault_nonce],
                ];
                let game_signer = &[&seeds[..]];
                let lamports = ctx.accounts.vault.to_account_info().lamports();
                let mut hasher = Sha256::new();
                hasher.update(
                            ctx.accounts.game.account1.to_string()+ 
                            &ctx.accounts.game.deposit_date1.to_string()+ 
                            &ctx.accounts.game.account2.to_string()+
                            &ctx.accounts.game.deposit_date2.to_string()+ 
                            &ctx.accounts.game.account3.to_string()+
                            &ctx.accounts.game.deposit_date3.to_string()+ 
                            &ctx.accounts.game.account4.to_string()+
                            &ctx.accounts.game.deposit_date4.to_string()+ 
                            &ctx.accounts.game.account5.to_string()+
                            &ctx.accounts.game.deposit_date5.to_string()+ 
                            &ctx.accounts.game.account6.to_string()+
                            &ctx.accounts.game.deposit_date6.to_string()+ 
                            &ctx.accounts.game.account7.to_string()+
                            &ctx.accounts.game.deposit_date7.to_string()+ 
                            &ctx.accounts.game.account8.to_string()+
                            &ctx.accounts.game.deposit_date8.to_string()+ 
                            &ctx.accounts.game.account9.to_string()+
                            &ctx.accounts.game.deposit_date9.to_string()+ 
                            &ctx.accounts.game.account10.to_string()+
                            &ctx.accounts.game.deposit_date10.to_string()
                        );
                let digest = hasher.finalize();

                let mut string_flag: u8 = digest[30].checked_div(16).unwrap();
                msg!("string_flag: {:?}", string_flag);
                if string_flag > 9 {
                    for i in 1..31 {
                        string_flag = digest[30 - i] % 16;
                        if string_flag <= 9 {
                            break;
                        }

                        string_flag = digest[30 - i].checked_div(16).unwrap();
                        if string_flag <= 9 {
                            break;
                        }
                    }
                }

                msg!("string_flag: {:?}", string_flag);
                for _i in 0..4 {
                    string_flag = string_flag + 1;
                    if string_flag > 9 {
                        string_flag = 0;
                    }
                }

                msg!("string_flag: {:?}", string_flag);
                let mut first_reciever = &ctx.accounts.depositor;
                if string_flag == 1 {
                    first_reciever = &ctx.accounts.account1;
                } else if string_flag == 2 {
                    first_reciever = &ctx.accounts.account2;
                } else if string_flag == 3 {
                    first_reciever = &ctx.accounts.account3;
                } else if string_flag == 4 {
                    first_reciever = &ctx.accounts.account4;
                } else if string_flag == 5 {
                    first_reciever = &ctx.accounts.account5;
                } else if string_flag == 6 {
                    first_reciever = &ctx.accounts.account6;
                } else if string_flag == 7 {
                    first_reciever = &ctx.accounts.account7;
                } else if string_flag == 8 {
                    first_reciever = &ctx.accounts.account8;
                } else if string_flag == 9 {
                    first_reciever = &ctx.accounts.account9;
                }

                for _i in 0..5 {
                    string_flag = string_flag + 1;
                    if string_flag > 9 {
                        string_flag = 0;
                    }
                }

                msg!("string_flag: {:?}", string_flag);
                let mut second_reciever = &ctx.accounts.depositor;
                if string_flag == 1 {
                    second_reciever = &ctx.accounts.account1;
                } else if string_flag == 2 {
                    second_reciever = &ctx.accounts.account2;
                } else if string_flag == 3 {
                    second_reciever = &ctx.accounts.account3;
                } else if string_flag == 4 {
                    second_reciever = &ctx.accounts.account4;
                } else if string_flag == 5 {
                    second_reciever = &ctx.accounts.account5;
                } else if string_flag == 6 {
                    second_reciever = &ctx.accounts.account6;
                } else if string_flag == 7 {
                    second_reciever = &ctx.accounts.account7;
                } else if string_flag == 8 {
                    second_reciever = &ctx.accounts.account8;
                } else if string_flag == 9 {
                    second_reciever = &ctx.accounts.account9;
                }

                msg!("game10 vault lamports: {:?}", lamports);

                anchor_lang::solana_program::program::invoke_signed(
                    &anchor_lang::solana_program::system_instruction::transfer(
                        &ctx.accounts.vault.key(), 
                        &first_reciever.key(), 
                        lamports.checked_div(2).unwrap()
                    ),
                    &[
                        ctx.accounts.vault.to_account_info(),
                        first_reciever.to_account_info(),
                    ],
                    game_signer,
                )?;
        
                anchor_lang::solana_program::program::invoke_signed(
                    &anchor_lang::solana_program::system_instruction::transfer(
                        &ctx.accounts.vault.key(), 
                        &second_reciever.key(), 
                        lamports.checked_div(2).unwrap()
                    ),
                    &[
                        ctx.accounts.vault.to_account_info(),
                        second_reciever.to_account_info(),
                    ],
                    game_signer,
                )?;

                // ctx.accounts.game.account1 = ctx.accounts.game.vault;
                // ctx.accounts.game.account2 = ctx.accounts.game.vault;
                // ctx.accounts.game.account3 = ctx.accounts.game.vault;
                // ctx.accounts.game.account4 = ctx.accounts.game.vault;
                // ctx.accounts.game.account5 = ctx.accounts.game.vault;
                // ctx.accounts.game.account6 = ctx.accounts.game.vault;
                // ctx.accounts.game.account7 = ctx.accounts.game.vault;
                // ctx.accounts.game.account8 = ctx.accounts.game.vault;
                // ctx.accounts.game.account9 = ctx.accounts.game.vault;
                // ctx.accounts.game.account10 = ctx.accounts.game.vault;
                // ctx.accounts.game.account_deposited1 = false;
                // ctx.accounts.game.account_deposited2 = false;
                // ctx.accounts.game.account_deposited3 = false;
                // ctx.accounts.game.account_deposited4 = false;
                // ctx.accounts.game.account_deposited5 = false;
                // ctx.accounts.game.account_deposited6 = false;
                // ctx.accounts.game.account_deposited7 = false;
                // ctx.accounts.game.account_deposited8 = false;
                // ctx.accounts.game.account_deposited9 = false;
                // ctx.accounts.game.account_deposited10 = false;
                ctx.accounts.game.last_hash = digest.as_slice().try_into().expect("Wrong length");
                
                ctx.accounts.pool.game10_finished = ctx.accounts.game.id.clone();
            }

        Ok(())
    }

    pub fn deposit_odd(ctx: Context<DepositOdd>) -> ProgramResult {
        let game = &mut ctx.accounts.game;
        if game.finished == true {
            return Err(ErrorCode::FinishedGame.into());
        }
        let ix = anchor_lang::solana_program::system_instruction::transfer(
                                    &ctx.accounts.depositor.key(), 
                                    &ctx.accounts.vault.key(), 
                                    game.bid);
        anchor_lang::solana_program::program::invoke(&ix, &[
                                                                ctx.accounts.depositor.to_account_info(), 
                                                                ctx.accounts.vault.to_account_info(), 
                                                            ])?;

        let current_time = clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();
        game.depositors[game.deposited] = ctx.accounts.depositor.key();
        game.deposit_dates[game.deposited] = current_time;
        game.deposited = game.deposited + 1;
        if game.deposited == game.players {
            game.finished = true;

            let mut hasher = Sha256::new();
            let mut hasher_update_str = "".to_string();
            for i in 0..game.players {
                hasher_update_str = hasher_update_str + game.depositors[i].to_string() + &game.deposit_dates[i].to_string();
            }
            hasher.update(hasher_update_str);
            let digest = hasher.finalize();

            let mut string_flag: u8 = digest[30].checked_div(16).unwrap();
            msg!("string_flag: {:?}", string_flag);
            if string_flag > 9 {
                for i in 1..31 {
                    string_flag = digest[30 - i] % 16;
                    if string_flag <= 9 {
                        break;
                    }

                    string_flag = digest[30 - i].checked_div(16).unwrap();
                    if string_flag <= 9 {
                        break;
                    }
                }
            }

            msg!("string_flag: {:?}", string_flag);
            for i in 0..game.players {
                if i % game.odd == 0 {
                    game.winners.push(game.depositors[string_flag as usize]);
                    if string_flag + game.odd > game.players - 1 {
                        string_flag = string_flag + game.odd - game.players;
                    } else {
                        string_flag = string_flag + game.odd;
                    }
                }
            }

            game.last_hash = digest.as_slice().try_into().expect("Wrong length");    
            pool.odd_game_finished = game.id.clone();
        }
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(nonce: u8)]
pub struct Initialize<'info> {
    authority: UncheckedAccount<'info>,

    #[account(
        seeds = [
            pool.to_account_info().key.as_ref()
        ],
        bump = nonce,
    )]
    pool_signer: UncheckedAccount<'info>,

    #[account(
        zero,
    )]
    pool: Box<Account<'info, Pool>>,

    owner: Signer<'info>,
    
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nonce: u8, vault_nonce: u8, id: String)]
pub struct CreateGame<'info> {
    #[account(mut)]
    pool: Account<'info, Pool>,
    #[account(
        init,
        payer = signer,
        seeds = [
            pool.key().as_ref(),
            "game".as_bytes(),
            id.as_bytes(),
        ],
        bump = nonce,
        space = 8 + 32*5 + 8*5 + 8*32 + 640
    )]
    game: Box<Account<'info, Game>>,
    #[account(
        seeds = [
            pool.to_account_info().key.as_ref(),
            "vault".as_bytes(),
            id.as_bytes(),
        ],
        bump = vault_nonce,
    )]
    vault: UncheckedAccount<'info>,
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nonce: u8, vault_nonce: u8, id: String)]
pub struct CreateGame5<'info> {
    #[account(mut)]
    pool: Account<'info, Pool>,
    #[account(
        init,
        payer = signer,
        seeds = [
            pool.key().as_ref(),
            "game5".as_bytes(),
            id.as_bytes(),
        ],
        bump = nonce,
        space = 8 + 32*7 + 8*7 + 8*32 + 640
    )]
    game: Box<Account<'info, Game5>>,
    signer: Signer<'info>,
    #[account(
        seeds = [
            pool.to_account_info().key.as_ref(),
            "vault5".as_bytes(),
            id.as_bytes(),
        ],
        bump = vault_nonce,
    )]
    vault: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nonce: u8, vault_nonce: u8, id: String)]
pub struct CreateGame10<'info> {
    #[account(mut)]
    pool: Account<'info, Pool>,
    #[account(
        init,
        payer = signer,
        seeds = [
            pool.key().as_ref(),
            "game10".as_bytes(),
            id.as_bytes(),
        ],
        bump = nonce,
        space = 8 + 32*12 + 8*12 + 8*32 + 640
    )]
    game: Box<Account<'info, Game10>>,
    signer: Signer<'info>,
    #[account(
        seeds = [
            pool.to_account_info().key.as_ref(),
            "vault10".as_bytes(),
            id.as_bytes(),
        ],
        bump = vault_nonce,
    )]
    vault: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct CreateOddGame<'info> {
    #[account(mut)]
    pool: Account<'info, Pool>,
    #[account(
        init,
        payer = signer,
        seeds = [
            pool.key().as_ref(),
            "odd_game".as_bytes(),
            id.as_bytes(),
        ],
    )]
    game: Box<Account<'info, OddGame>>,
    #[account(
        seeds = [
            pool.to_account_info().key.as_ref(),
            "odd_vault".as_bytes(),
            id.as_bytes(),
        ],
    )]
    vault: UncheckedAccount<'info>,
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pool: Account<'info, Pool>,
    #[account(
        mut, 
        has_one = vault,
    )]
    game: Box<Account<'info, Game>>,
    #[account(
        mut,
        seeds = [
            pool.key().as_ref(),
            "vault".as_bytes(),
            game.id.as_bytes(),
        ],
        bump = game.vault_nonce,
    )]
    vault: AccountInfo<'info>,
    #[account(mut)]
    account1: AccountInfo<'info>,
    #[account(mut)]
    account2: AccountInfo<'info>,
    #[account(mut)]
    account3: AccountInfo<'info>,
    #[account(mut)]
    depositor: AccountInfo<'info>,
    #[account(
        seeds = [
            pool.key().as_ref(),
        ],
        bump = pool.nonce,
    )]
    pool_signer: UncheckedAccount<'info>,
    // token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit5<'info> {
    #[account(mut)]
    pool: Account<'info, Pool>,
    #[account(
        mut, 
        has_one = vault,
    )]
    game: Box<Account<'info, Game5>>,
    #[account(
        mut,
        seeds = [
            pool.key().as_ref(),
            "vault5".as_bytes(),
            game.id.as_bytes(),
        ],
        bump = game.vault_nonce,
    )]
    vault: AccountInfo<'info>,
    #[account(mut)]
    account1: AccountInfo<'info>,
    #[account(mut)]
    account2: AccountInfo<'info>,
    #[account(mut)]
    account3: AccountInfo<'info>,
    #[account(mut)]
    account4: AccountInfo<'info>,
    #[account(mut)]
    account5: AccountInfo<'info>,
    #[account(mut)]
    depositor: AccountInfo<'info>,
    #[account(
        seeds = [
            pool.key().as_ref(),
        ],
        bump = pool.nonce,
    )]
    pool_signer: UncheckedAccount<'info>,
    // token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit10<'info> {
    #[account(mut)]
    pool: Account<'info, Pool>,
    #[account(
        mut, 
        has_one = vault,
    )]
    game: Box<Account<'info, Game10>>,
    #[account(
        mut,
        seeds = [
            pool.key().as_ref(),
            "vault".as_bytes(),
            game.id.as_bytes(),
        ],
        bump = game.vault_nonce,
    )]
    vault: AccountInfo<'info>,
    #[account(mut)]
    account1: AccountInfo<'info>,
    #[account(mut)]
    account2: AccountInfo<'info>,
    #[account(mut)]
    account3: AccountInfo<'info>,
    #[account(mut)]
    account4: AccountInfo<'info>,
    #[account(mut)]
    account5: AccountInfo<'info>,
    #[account(mut)]
    account6: AccountInfo<'info>,
    #[account(mut)]
    account7: AccountInfo<'info>,
    #[account(mut)]
    account8: AccountInfo<'info>,
    #[account(mut)]
    account9: AccountInfo<'info>,
    #[account(mut)]
    account10: AccountInfo<'info>,
    #[account(mut)]
    depositor: AccountInfo<'info>,
    #[account(
        seeds = [
            pool.key().as_ref(),
        ],
        bump = pool.nonce,
    )]
    pool_signer: UncheckedAccount<'info>,
    // token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositOdd<'info> {
    #[account(mut)]
    pool: Account<'info, Pool>,
    #[account(
        mut, 
        has_one = vault,
    )]
    game: Box<Account<'info, Game>>,
    #[account(
        mut,
        seeds = [
            pool.key().as_ref(),
            "odd_vault".as_bytes(),
            game.id.as_bytes(),
        ],
        bump = game.vault_nonce,
    )]
    vault: AccountInfo<'info>,
    #[account(mut)]
    depositor: AccountInfo<'info>,
    #[account(
        seeds = [
            pool.key().as_ref(),
        ],
        bump = pool.nonce,
    )]
    pool_signer: UncheckedAccount<'info>,
    // token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Pool {
    pub game_count: u64,
    pub game5_count: u64,
    pub game10_count: u64,
    pub odd_game_count: u64,
    pub game_finished: String,
    pub game5_finished: String,
    pub game10_finished: String,
    pub odd_game_finished: String,
    pub nonce: u8,
    /// Priviledged account.
    pub authority: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Game {
    pub vault: Pubkey,
    pub creator: Pubkey,
    pub account1: Pubkey,
    pub account2: Pubkey,
    pub account3: Pubkey,
    pub account_deposited1: bool,
    pub account_deposited2: bool,
    pub account_deposited3: bool,
    pub deposit_date1: u64,
    pub deposit_date2: u64,
    pub deposit_date3: u64,
    pub deposit_amount1: u64,
    pub deposit_amount2: u64,
    pub deposit_amount3: u64,
    pub nonce: u8,
    pub vault_nonce: u8,
    pub id: String,
    pub last_hash: [u8; 32],
    /// Priviledged account.
    pub authority: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Game5 {
    pub vault: Pubkey,
    pub creator: Pubkey,
    pub account1: Pubkey,
    pub account2: Pubkey,
    pub account3: Pubkey,
    pub account4: Pubkey,
    pub account5: Pubkey,
    pub account_deposited1: bool,
    pub account_deposited2: bool,
    pub account_deposited3: bool,
    pub account_deposited4: bool,
    pub account_deposited5: bool,
    pub deposit_date1: u64,
    pub deposit_date2: u64,
    pub deposit_date3: u64,
    pub deposit_date4: u64,
    pub deposit_date5: u64,
    pub deposit_amount1: u64,
    pub deposit_amount2: u64,
    pub deposit_amount3: u64,
    pub deposit_amount4: u64,
    pub deposit_amount5: u64,
    pub nonce: u8,
    pub id: String,
    pub vault_nonce: u8,
    pub last_hash: [u8; 32],
    /// Priviledged account.
    pub authority: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Game10 {
    pub vault: Pubkey,
    pub creator: Pubkey,
    pub account1: Pubkey,
    pub account2: Pubkey,
    pub account3: Pubkey,
    pub account4: Pubkey,
    pub account5: Pubkey,
    pub account6: Pubkey,
    pub account7: Pubkey,
    pub account8: Pubkey,
    pub account9: Pubkey,
    pub account10: Pubkey,
    pub account_deposited1: bool,
    pub account_deposited2: bool,
    pub account_deposited3: bool,
    pub account_deposited4: bool,
    pub account_deposited5: bool,
    pub account_deposited6: bool,
    pub account_deposited7: bool,
    pub account_deposited8: bool,
    pub account_deposited9: bool,
    pub account_deposited10: bool,
    pub deposit_date1: u64,
    pub deposit_date2: u64,
    pub deposit_date3: u64,
    pub deposit_date4: u64,
    pub deposit_date5: u64,
    pub deposit_date6: u64,
    pub deposit_date7: u64,
    pub deposit_date8: u64,
    pub deposit_date9: u64,
    pub deposit_date10: u64,
    pub deposit_amount1: u64,
    pub deposit_amount2: u64,
    pub deposit_amount3: u64,
    pub deposit_amount4: u64,
    pub deposit_amount5: u64,
    pub deposit_amount6: u64,
    pub deposit_amount7: u64,
    pub deposit_amount8: u64,
    pub deposit_amount9: u64,
    pub deposit_amount10: u64,
    pub nonce: u8,
    pub id: String,
    pub vault_nonce: u8,
    pub last_hash: [u8; 32],
    /// Priviledged account.
    pub authority: Pubkey,
}

#[account]
#[derive(Default)]
pub struct OddGame {
    pub vault: Pubkey,
    pub finished: bool,
    pub odd: u8,
    pub players: u8,
    pub bid: u64,
    pub creator: Pubkey,
    pub deposited: u8,
    pub nonce: u8,
    pub id: String,
    pub vault_nonce: u8,
    pub last_hash: [u8; 32],
    pub depositors: [Pubkey; 100],
    pub deposit_dates: [u64; 100],
    pub winners: [Pubkey; 50],
    /// Priviledged account.
    pub authority: Pubkey,
}


#[error]
pub enum ErrorCode {
    #[msg("Minimize deposit amount is 0.000001 SOL.")]
    MinDepositAmount,
    #[msg("This address deposited already.")]
    AlreadyDeposit,
    #[msg("Depositor address does not registered.")]
    DepositorNotMatch,
    #[msg("Finished Game.")]
    FinishedGame,
}
