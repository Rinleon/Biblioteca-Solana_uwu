use anchor_lang::prelude::*;

declare_id!(**);
#[program]
pub mod juegos{
    use super::*;

    pub fn correr_juego() ->  Result<()>{

    }

}
#[Account]
#[deriveve(InitSpace)]
pub struct Tjuego{
    cuenta: Pubkey,
    #{max_len(60)}
    nombre: String,

    juegos: Vec<Juego>,
}

#[derive(AnchorDeserialize,AnchorDeserialize,Clone,InitSpace,PartialEq,Debug)]
pub struct Juego{
    #{amx_len(60)}
    nombre:String,
    precio:u16,
    #{amx_len(60)}
    plataforma: String,
    #{amx_len(30)}
    genero: String,

    disoinible: bool,
}

///Contexto

#[derive(Accounts)]
pub struct NuevoTjuego{
    #{account(mut)}
    pub cuenta:Signer<'info>,
    pub jjuego:AAccount<'info,Tjuego>,
    pub system_program:Program<'info,System>,
}

pub struct Nuevojuego{
     #[account(
        init,
        payer = cuenta,
        space = Tjuego::INTt_SPACE + 8,
        seeds = [b"jjuego",cuenta.key().as_ref()]
        bump
     )]
    pub cuenta:Signer<'info>,


    #[account(mut)]
    pub jjuego:Account<'info, Tjuego>,
}
