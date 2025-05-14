use anchor_lang::prelude::*;

declare_id!("FaKjDPDkAVgygnptmnBLpZyjKdsGCcCxuhNcGGL4baJ7");

// anchor discriminator describes the type of program it is.
pub const ANCHOR_DISCRIMINATOR_SIZE : usize = 8;

#[program]
pub mod favorite_game {
    use super::*;

    pub fn set_favorites(ctx: Context<SetFavorites> , number : u64 , color : String , hobbies : Vec<String>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id); // msg is just like console log
        let user_public_key = ctx.accounts.user.key();
        
        msg!("User {user_public_key}'s favorite number is {number} 
        and favorite color is {color} and their hobbies are {hobbieis:?}");
        
        ctx.accounts.favorites.set_inner(Favorites { number, color, hobbies });
        Ok(())
    }

}



#[account]
#[derive(InitSpace)] // to know the size of the program
pub struct Favorites{

    pub number : u64,
    #[max_len(50)]
    pub color : String,
    #[max_len(5,50)]
    pub hobbies : Vec<String>
}

// Struct of accounts for setfavorites instruction handler
// Soooo... the convention is to set the name as function itself , nothing fancy dude..
#[derive(Accounts)]
pub struct SetFavorites<'info>{
    #[account(mut)] // signer going to pay for the account creation during setting fav.
    pub user : Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space=ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds =[b"favorites",user.key().as_ref()],
        bump 


    )]
    pub favorites : Account<'info,Favorites>,

    pub system_program : Program<'info , System>
}