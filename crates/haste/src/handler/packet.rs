use std::marker::PhantomData;

use crate::{
    parser::Context,
    protos::{self, prost::Message},
};

pub trait FromPacket: Sized {
    type Item<'a>;

    fn from_context<'a>(
        context: &'a Context,
        packet_type: u32,
        data: &'a [u8],
    ) -> Option<Self::Item<'a>>;
}

impl FromPacket for &Context {
    type Item<'a> = &'a Context;

    fn from_context<'a>(
        context: &'a Context,
        _packet_type: u32,
        _data: &'a [u8],
    ) -> Option<&'a Context> {
        Some(context)
    }
}

impl FromPacket for protos::CCitadelUserMsgHeroKilled {
    type Item<'a> = Self;

    fn from_context<'a>(
        _context: &'a Context,
        packet_type: u32,
        data: &'a [u8],
    ) -> Option<Self::Item<'a>> {
        if protos::CitadelUserMessageIds::KEUserMsgHeroKilled as u32 == packet_type {
            Self::decode(data).ok()
        } else {
            None
        }
    }
}
impl FromPacket for protos::CdotaUserMsgChatMessage {
    type Item<'a> = Self;

    fn from_context<'a>(
        _context: &'a Context,
        packet_type: u32,
        data: &'a [u8],
    ) -> Option<Self::Item<'a>> {
        if protos::EDotaUserMessages::DotaUmChatMessage as u32 == packet_type {
            Self::decode(data).ok()
        } else {
            None
        }
    }
}

pub trait IntoPacketHandler<S, PARAMS> {
    fn wrap<F>(func: F) -> PacketHandlerWrapper<S, F, PARAMS> {
        PacketHandlerWrapper {
            func,
            _phantom: Default::default(),
        }
    }
}

impl<S, F> IntoPacketHandler<S, ()> for F where F: Fn() {}
macro_rules! impl_into_packet_handler {
    (
        $($ty:ident),*
    ) => {
        impl<S, F, $($ty,)*> IntoPacketHandler<S, ($($ty,)*)> for F where F: Fn(&mut S, $($ty,)*) {}
    };
}

pub trait PacketHandler<S> {
    fn call(&self, state: &mut S, context: &Context, packet_type: u32, data: &[u8]);
}

pub struct PacketHandlerWrapper<S, F, PARAMS> {
    func: F,
    _phantom: PhantomData<(S, PARAMS)>,
}

impl<S, F> PacketHandler<S> for PacketHandlerWrapper<S, F, ()>
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
        impl<F, S, $($ty,)*> PacketHandler<S> for PacketHandlerWrapper<S, F, ($($ty,)*)>
        where
            S: 'static,
            F: Fn(&mut S, $(<$ty as FromPacket>::Item<'_>,)*) + 'static,
            $( $ty: FromPacket + 'static, )*
        {
            fn call(
                &self,
                state: &mut S,
                context: &Context,
                packet_type: u32,
                data: &[u8],
            ) {
                $(
                    let Some($ty) = $ty::from_context(context, packet_type, data) else {
                        return;
                    };
                )*

                (self.func)(state, $($ty,)*);
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
