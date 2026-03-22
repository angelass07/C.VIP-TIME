use anchor_lang::prelude::*;

declare_id!("38dxXpL8wvgzr4Ai12nvpRDdEw3RzVvBgh5CgBqjYyx6");

#[program]
pub mod cvip_time {
    use super::*;

    pub fn crear_ticket(
        ctx: Context<CrearTicket>,
        peli: String,
        asiento: String,
        pedido: String,
    ) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket;

        ticket.peli = peli;
        ticket.asiento = asiento;
        ticket.pedido = pedido;
        ticket.confirmado_en_sala = false;
        ticket.estado = 0;
        ticket.atendido_por = "Esperando asignación".to_string();

        Ok(())
    }

    pub fn confirmar_llegada(ctx: Context<UpdateTicket>) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket;
        ticket.confirmado_en_sala = true;
        Ok(())
    }

    pub fn actualizar_estado(
        ctx: Context<UpdateTicket>,
        nuevo_estado: u8,
        staff_nombre: String,
    ) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket;

        if nuevo_estado == 1 && !ticket.confirmado_en_sala {
            return Err(error!(Errores::ClienteNoHaLlegado));
        }

        ticket.estado = nuevo_estado;
        ticket.atendido_por = staff_nombre;

        Ok(())
    }
}

#[account]
pub struct TicketVIP {
    pub peli: String,
    pub asiento: String,
    pub pedido: String,
    pub atendido_por: String,
    pub confirmado_en_sala: bool,
    pub estado: u8,
}

#[derive(Accounts)]
pub struct CrearTicket<'info> {
    #[account(init, payer = user, space = 8 + 300)]
    pub ticket: Account<'info, TicketVIP>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTicket<'info> {
    #[account(mut)]
    pub ticket: Account<'info, TicketVIP>,
}

#[error_code]
pub enum Errores {
    #[msg("El cliente debe marcar 'Ya llegué' para poder cocinar.")]
    ClienteNoHaLlegado,
}
