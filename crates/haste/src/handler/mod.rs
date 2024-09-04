use crate::{
    // entities::{Entity, UpdateType},
    parser::{self, Context, Visitor},
};

pub mod packet;
use packet::PacketHandler;

use self::packet::{IntoPacketHandler, PacketHandlerWrapper};

#[derive(Default)]
pub struct HandlerVisitor<S> {
    packet_handlers: Vec<Box<dyn PacketHandler<S>>>,
    // entity_handlers: Vec<dyn EntityHandler>,
    state: S,
}

impl<S: 'static> HandlerVisitor<S> {
    pub fn with_state(state: S) -> Self {
        Self {
            packet_handlers: vec![],
            // entity_handlers: vec![],
            state,
        }
    }

    pub fn with<PARAMS, H>(mut self, handler: H) -> Self
    where
        PARAMS: 'static,
        H: IntoPacketHandler<S, PARAMS> + 'static,
        PacketHandlerWrapper<S, H, PARAMS>: PacketHandler<S>,
    {
        let handler = Box::new(<H as IntoPacketHandler<S, PARAMS>>::wrap(handler))
            as Box<dyn PacketHandler<S>>;

        self.packet_handlers.push(handler);
        self
    }
}

impl<S> Visitor for HandlerVisitor<S> {
    fn on_packet(&mut self, ctx: &Context, packet_type: u32, data: &[u8]) -> parser::Result<()> {
        for handler in &self.packet_handlers {
            handler.call(&mut self.state, ctx, packet_type, data);
        }

        Ok(())
    }

    // fn on_entity(
    //     &mut self,
    //     ctx: &Context,
    //     update_flags: usize,
    //     update_type: UpdateType,
    //     // TODO: include updated fields (list of field paths?)
    //     entity: &Entity,
    // ) -> parser::Result<()> {
    //     for handler in self.entity_handlers {
    //         //
    //     }

    //     Ok(())
    // }
}
