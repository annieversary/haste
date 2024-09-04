use std::{marker::PhantomData, num::Wrapping};

use crate::{
    parser::Context,
    protos::{self, prost::Message},
};

pub trait FromPacket<S>: Sized {
    fn from_context(
        state: &mut S,
        context: &Context,
        packet_type: u32,
        data: &[u8],
    ) -> Option<Self>;
}

impl<'a, S> FromPacket<S> for &'a Context {
    fn from_context(
        _state: &mut S,
        context: &'a Context,
        _packet_type: u32,
        _data: &[u8],
    ) -> Option<Self> {
        Some(context)
    }
}

impl<'a, S> FromPacket<S> for &'a mut S {
    fn from_context(
        state: &'a mut S,
        _context: &Context,
        _packet_type: u32,
        _data: &[u8],
    ) -> Option<Self> {
        Some(state)
    }
}

impl<S> FromPacket<S> for protos::CCitadelUserMsgHeroKilled {
    fn from_context(
        _state: &mut S,
        _context: &Context,
        packet_type: u32,
        data: &[u8],
    ) -> Option<Self> {
        if protos::CitadelUserMessageIds::KEUserMsgHeroKilled as u32 == packet_type {
            Self::decode(data).ok()
        } else {
            None
        }
    }
}
impl<S> FromPacket<S> for protos::CdotaUserMsgChatMessage {
    fn from_context(
        _state: &mut S,
        _context: &Context,
        packet_type: u32,
        data: &[u8],
    ) -> Option<Self> {
        if protos::EDotaUserMessages::DotaUmChatMessage as u32 == packet_type {
            Self::decode(data).ok()
        } else {
            None
        }
    }
}

pub trait IntoPacketHandler<PARAMS> {
    fn wrap<F>(func: F) -> PacketHandlerWrapper<F, PARAMS> {
        PacketHandlerWrapper {
            func,
            _phantom: Default::default(),
        }
    }
}

impl<F> IntoPacketHandler<()> for F where F: Fn() {}
impl<F, T1> IntoPacketHandler<(T1,)> for F where F: Fn(T1) {}
impl<F, T1, T2, T3> IntoPacketHandler<(T1, T2, T3)> for F where F: Fn(T1, T2, T3) {}

pub trait PacketHandler<S> {
    fn call(&self, state: &mut S, context: &Context, packet_type: u32, data: &[u8]);
}

pub(crate) struct PacketHandlerWrapper<F, PARAMS> {
    func: F,
    _phantom: PhantomData<PARAMS>,
}

impl<S, F> PacketHandler<S> for PacketHandlerWrapper<F, ()>
where
    F: Fn() + 'static,
{
    fn call(&self, _state: &mut S, _context: &Context, _packet_type: u32, _data: &[u8]) {
        (self.func)();
    }
}
impl<F, S, T1> PacketHandler<S> for PacketHandlerWrapper<F, (T1,)>
where
    F: Fn(T1) + 'static,
    T1: FromPacket<S> + 'static,
{
    fn call(&self, state: &mut S, context: &Context, packet_type: u32, data: &[u8]) {
        let Some(t1) = T1::from_context(state, context, packet_type, data) else {
            return;
        };

        (self.func)(t1);
    }
}
impl<F, S, T1, T2, T3> PacketHandler<S> for PacketHandlerWrapper<F, (T1, T2, T3)>
where
    F: Fn(T1, T2, T3) + 'static,
    T1: FromPacket<S> + 'static,
    T2: FromPacket<S> + 'static,
    T3: FromPacket<S> + 'static,
{
    fn call(&self, state: &mut S, context: &Context, packet_type: u32, data: &[u8]) {
        let Some(t1) = T1::from_context(state, context, packet_type, data) else {
            return;
        };
        let Some(t2) = T2::from_context(state, context, packet_type, data) else {
            return;
        };
        let Some(t3) = T3::from_context(state, context, packet_type, data) else {
            return;
        };

        (self.func)(t1, t2, t3);
    }
}

// TODO add all the other impls, using a macro
