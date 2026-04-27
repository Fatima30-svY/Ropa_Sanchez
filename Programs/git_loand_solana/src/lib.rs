use anchor_lang::prelude::*;

declare_id!("6EDJnxGmFkWTHqGyXJZ4RowukqBewYC3Tc845X6xvVjg");

#[program]
pub mod tienda_ropa {
    use super::*;

    // 1. CREATE (PDA): Inicializa la tienda
    pub fn crear_tienda(ctx: Context<NuevaTienda>, nombre: String) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;
        tienda.owner = ctx.accounts.owner.key();
        tienda.nombre = nombre;
        tienda.prendas = Vec::new();

        msg!("Tienda '{}' creada exitosamente.", tienda.nombre);
        Ok(())
    }

    // 2. CREATE (Dato): Agrega una prenda al catálogo
    pub fn agregar_prenda(
        ctx: Context<ModificarTienda>,
        nombre: String,
        talla: String,
        precio: u64,
    ) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;
        require!(tienda.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let nueva_prenda = Prenda {
            nombre,
            talla,
            precio,
            disponible: true,
        };

        tienda.prendas.push(nueva_prenda);
        msg!("Prenda agregada exitosamente.");
        Ok(())
    }

    // 3. UPDATE: Modifica talla, precio y disponibilidad de una prenda
    pub fn editar_prenda(
        ctx: Context<ModificarTienda>,
        nombre: String,
        nueva_talla: String,
        nuevo_precio: u64,
        nueva_disponibilidad: bool,
    ) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;
        require!(tienda.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let lista = &mut tienda.prendas;
        for i in 0..lista.len() {
            if lista[i].nombre == nombre {
                lista[i].talla = nueva_talla;
                lista[i].precio = nuevo_precio;
                lista[i].disponible = nueva_disponibilidad;
                msg!("Prenda '{}' actualizada.", nombre);
                return Ok(());
            }
        }
        Err(Errores::PrendaNoExiste.into())
    }

    // 4. DELETE: Elimina una prenda del catálogo
    pub fn eliminar_prenda(ctx: Context<ModificarTienda>, nombre: String) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;
        require!(tienda.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let lista = &mut tienda.prendas;
        let index = lista.iter().position(|p| p.nombre == nombre); // Patrón tomado del laboratorio

        if let Some(i) = index {
            lista.remove(i);
            msg!("Prenda '{}' eliminada del catálogo.", nombre);
            Ok(())
        } else {
            Err(Errores::PrendaNoExiste.into())
        }
    }

    // 5. READ: Muestra el catálogo completo
    pub fn ver_catalogo(ctx: Context<ModificarTienda>) -> Result<()> {
        msg!("Tienda: {}", ctx.accounts.tienda.nombre);
        msg!("Catálogo actual: {:#?}", ctx.accounts.tienda.prendas);
        Ok(())
    }
}

// --- STRUCTS ---

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Prenda {
    #[max_len(60)]
    pub nombre: String,
    #[max_len(5)]
    pub talla: String,
    pub precio: u64,
    pub disponible: bool,
}

#[account]
#[derive(InitSpace)]
pub struct Tienda {
    pub owner: Pubkey,
    #[max_len(60)]
    pub nombre: String,
    #[max_len(20)]
    pub prendas: Vec<Prenda>,
}

// --- CONTEXTOS ---

#[derive(Accounts)]
pub struct NuevaTienda<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + Tienda::INIT_SPACE,
        seeds = [b"tienda", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tienda>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarTienda<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]  // ← solo mut, sin seeds ni bump
    pub tienda: Account<'info, Tienda>,
}

// --- ERRORES ---

#[error_code]
pub enum Errores {
    #[msg("No tienes permisos sobre esta tienda.")]
    NoEresElOwner,
    #[msg("La prenda no existe en el catálogo.")]
    PrendaNoExiste,
}
