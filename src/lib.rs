use anchor_lang::prelude::*;

// Pega aquí el ID que te genere Playground al hacer Deploy
declare_id!("ECWvgLE1eskduxor73q14cT6YvttLNWZjBXSPxRt1pV4");

#[program]
pub mod gestor_tareas {
    use super::*;

    pub fn crear_tarea(ctx: Context<CrearTarea>, descripcion: String) -> Result<()> {
        let tarea = &mut ctx.accounts.tarea;
        tarea.autor = *ctx.accounts.usuario.key;
        tarea.descripcion = descripcion;
        tarea.completada = false;
        msg!("Tarea creada exitosamente");
        Ok(())
    }

    pub fn marcar_completada(ctx: Context<ActualizarTarea>) -> Result<()> {
        let tarea = &mut ctx.accounts.tarea;
        tarea.completada = true;
        msg!("Estado actualizado a completada");
        Ok(())
    }

    pub fn borrar_tarea(_ctx: Context<BorrarTarea>) -> Result<()> {
        msg!("Cuenta cerrada, SOL devueltos al usuario");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CrearTarea<'info> {
    #[account(
        init, 
        payer = usuario, 
        space = 8 + 32 + (4 + 100) + 1, 
        seeds = [b"tarea-usuario", usuario.key().as_ref()], 
        bump
    )]
    pub tarea: Account<'info, Tarea>,
    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarTarea<'info> {
    #[account(mut, seeds = [b"tarea-usuario", usuario.key().as_ref()], bump)]
    pub tarea: Account<'info, Tarea>,
    pub usuario: Signer<'info>,
}

#[derive(Accounts)]
pub struct BorrarTarea<'info> {
    #[account(
        mut, 
        close = usuario, 
        seeds = [b"tarea-usuario", usuario.key().as_ref()], 
        bump
    )]
    pub tarea: Account<'info, Tarea>,
    #[account(mut)]
    pub usuario: Signer<'info>,
}

#[account]
pub struct Tarea {
    pub autor: Pubkey,
    pub descripcion: String,
    pub completada: bool,
}