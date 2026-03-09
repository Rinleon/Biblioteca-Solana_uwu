use anchor_lang::prelude::*;

declare_id!("2sm1vNABdUrSB8tWC8FZksiC18S85mXUpA7tXtYcVNzK");

#[program]
pub mod juegos {
    use super::*;

    pub fn correr_juego(context: Context<NuevoTjuego>, nombre: String) -> Result<()> {

        let cuenta_id = context.accounts.cuenta.key();
        let juegos: Vec<Juego> = Vec::new();

        context.accounts.jjuego.set_inner(Tjuego{
            cuenta: cuenta_id,
            nombre,
            juegos,
        });

        Ok(())
    }

    pub fn agregar_juego(context: Context<Nuevojuego>,nombre: String,precio: u16,plataforma: String,genero: String) -> Result<()> {

        let juego: Juego = Juego{
            nombre,
            precio,
            plataforma,
            genero,
            disponible: true,
        };

        context.accounts.jjuego.juegos.push(juego);

        Ok(())
    }

    pub fn ver_juego(context: Context<Nuevojuego>) -> Result<()> {

        let juegos = &context.accounts.jjuego.juegos;

        msg!("Estos son los juegos: {:#?}", juegos);

        Ok(())
    }
    
    pub fn eliminar_juego(context: Context<Nuevojuego>, nombre: String) -> Result<()> {

    let juegos = &mut context.accounts.jjuego.juegos;

    for i in 0..juegos.len() {
        if juegos[i].nombre == nombre {

            juegos.remove(i);

            msg!("Este juego {} está game over", nombre);

            return Ok(());
        }
    }

    Err(Errores::JuegoNoExiste.into())
}


   pub fn cambios_juego(context: Context<Nuevojuego>, nombre: String) -> Result<()> {
    let juegos = &mut context.accounts.jjuego.juegos;
    for i in 0..juegos.len() {

        if juegos[i].nombre == nombre {

            let cambios = juegos[i].disponible;
            let nuevo_cambio = !cambios;
            juegos[i].disponible = nuevo_cambio;
            msg!("El juego {} fue modificado. Disponible: {}", nombre, nuevo_cambio);

            return Ok(());
        }
    }

    Err(Errores::JuegoNoExiste.into())
}
}


#[error_code]
pub enum Errores{
    #[msg("ERROR, QUIEN ERES TU!?????")]
    NoEresLacuenta,

    #[msg("Error ese Juego no existe :,c")]
    JuegoNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Tjuego{
    pub cuenta: Pubkey,

    #[max_len(10)]
    pub nombre: String,

    #[max_len(10)]
    pub juegos: Vec<Juego>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Juego{

    #[max_len(60)]
    pub nombre:String,

    pub precio:u16,

    #[max_len(60)]
    pub plataforma:String,

    #[max_len(30)]
    pub genero: String,

    pub disponible: bool,
}

/// Contexto

#[derive(Accounts)]
pub struct NuevoTjuego<'info>{

    #[account(mut)]
    pub cuenta: Signer<'info>,

    #[account(
        init,
        payer = cuenta,
        space = 8 + Tjuego::INIT_SPACE,
        seeds = [b"jjuego", cuenta.key().as_ref()],
        bump
     )]
    pub jjuego: Account<'info,Tjuego>,

    pub system_program: Program<'info,System>,
}

#[derive(Accounts)]
pub struct Nuevojuego<'info>{

    pub cuenta:Signer<'info>,

    #[account(mut)]
    pub jjuego:Account<'info, Tjuego>,
}
