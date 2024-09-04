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
macro_rules! impl_into_packet_handler {
    (
        $($ty:ident),*
    ) => {
        impl<F, $($ty,)*> IntoPacketHandler<($($ty,)*)> for F where F: Fn($($ty,)*) {}
    };
}

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

macro_rules! impl_packet_handler {
    (
        $($ty:ident),*
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<F, S, $($ty,)*> PacketHandler<S> for PacketHandlerWrapper<F, ($($ty,)*)>
        where
            F: Fn($($ty,)*) + 'static,
            $( $ty: FromPacket<S> + 'static, )*
        {
            fn call(&self, state: &mut S, context: &Context, packet_type: u32, data: &[u8]) {
                $(
                    let Some($ty) = $ty::from_context(state, context, packet_type, data) else {
                        return;
                    };
                )*

                (self.func)($($ty,)*);
            }
        }
    };
}

#[rustfmt::skip]
macro_rules! all_the_tuples {
    ($name:ident) => {
        $name!(T1);
        $name!(T1, T2);
        $name!(T1, T2, T3);
        $name!(T1, T2, T3, T4);
        $name!(T1, T2, T3, T4, T5);
        $name!(T1, T2, T3, T4, T5, T6);
        $name!(T1, T2, T3, T4, T5, T6, T7);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    };
}

all_the_tuples!(impl_into_packet_handler);
all_the_tuples!(impl_packet_handler);
