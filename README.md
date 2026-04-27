# 🏪 Tienda Ropa 

Este programa implementa un CRUD completo para administrar el catálogo de una tienda de ropa directamente en la blockchain de Solana.

---

### `crear_tienda`

```rust
pub fn crear_tienda(ctx: Context<NuevaTienda>, nombre: String) -> Result<()> {
    let tienda = &mut ctx.accounts.tienda;
    tienda.owner = ctx.accounts.owner.key();
    tienda.nombre = nombre;
    tienda.prendas = Vec::new();
    msg!("Tienda '{}' creada exitosamente.", tienda.nombre);
    Ok(())
}
```

Inicializa una nueva tienda. Asigna el `owner`, el nombre y un catálogo vacío. Solo se puede crear una tienda por wallet gracias a la PDA.

---

### `agregar_prenda`

```rust
pub fn agregar_prenda(ctx: Context<ModificarTienda>, nombre: String, talla: String, precio: u64) -> Result<()> {
    let tienda = &mut ctx.accounts.tienda;
    require!(tienda.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);
    let nueva_prenda = Prenda { nombre, talla, precio, disponible: true };
    tienda.prendas.push(nueva_prenda);
    Ok(())
}
```

Agrega una nueva prenda al catálogo. Verifica que quien firma sea el dueño y la crea con `disponible: true` por defecto.

---

### `editar_prenda`

```rust
pub fn editar_prenda(ctx: Context<ModificarTienda>, nombre: String, nueva_talla: String, nuevo_precio: u64, nueva_disponibilidad: bool) -> Result<()> {
    let lista = &mut tienda.prendas;
    for i in 0..lista.len() {
        if lista[i].nombre == nombre {
            lista[i].talla = nueva_talla;
            lista[i].precio = nuevo_precio;
            lista[i].disponible = nueva_disponibilidad;
            return Ok(());
        }
    }
    Err(Errores::PrendaNoExiste.into())
}
```

Busca una prenda por nombre y actualiza su talla, precio y disponibilidad. Si no la encuentra, regresa el error `PrendaNoExiste`.

---

### `eliminar_prenda`

```rust
pub fn eliminar_prenda(ctx: Context<ModificarTienda>, nombre: String) -> Result<()> {
    let lista = &mut tienda.prendas;
    let index = lista.iter().position(|p| p.nombre == nombre);
    if let Some(i) = index {
        lista.remove(i);
        Ok(())
    } else {
        Err(Errores::PrendaNoExiste.into())
    }
}
```

Busca la prenda por nombre con `.iter().position()` y la elimina con `.remove(i)`. Si no existe, regresa `PrendaNoExiste`.

---

### `ver_catalogo`

```rust
pub fn ver_catalogo(ctx: Context<ModificarTienda>) -> Result<()> {
    msg!("Tienda: {}", ctx.accounts.tienda.nombre);
    msg!("Catálogo actual: {:#?}", ctx.accounts.tienda.prendas);
    Ok(())
}
```

Imprime en los logs de la transacción el nombre de la tienda y todas las prendas con sus datos.

---

### Structs

```rust
pub struct Prenda {
    pub nombre: String,  // max 60 chars
    pub talla: String,   // max 5 chars
    pub precio: u64,
    pub disponible: bool,
}

pub struct Tienda {
    pub owner: Pubkey,
    pub nombre: String,  // max 60 chars
    pub prendas: Vec<Prenda>,  // max 20 prendas
}
```

`Prenda` representa cada artículo del catálogo. `Tienda` es la cuenta on-chain que agrupa al dueño, el nombre y el vector de prendas.

---

### Contextos

```rust
pub struct NuevaTienda<'info> {
    pub owner: Signer<'info>,
    #[account(init, payer = owner, space = 8 + Tienda::INIT_SPACE,
        seeds = [b"tienda", owner.key().as_ref()], bump)]
    pub tienda: Account<'info, Tienda>,
    pub system_program: Program<'info, System>,
}
```

Usado solo por `crear_tienda`. Inicializa la cuenta `tienda` como PDA con seeds `[b"tienda", owner]`, cobrando la renta al `owner`.

```rust
pub struct ModificarTienda<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub tienda: Account<'info, Tienda>,
}
```

Usado por el resto de instrucciones. Solo requiere que `tienda` sea mutable. La validación del dueño se hace manualmente con `require!` dentro de cada función.

---

### Errores

```rust
pub enum Errores {
    #[msg("No tienes permisos sobre esta tienda.")]
    NoEresElOwner,
    #[msg("La prenda no existe en el catálogo.")]
    PrendaNoExiste,
}
```

`NoEresElOwner` se lanza cuando quien firma no es el dueño registrado. `PrendaNoExiste` cuando se intenta editar o eliminar una prenda que no está en el catálogo.
