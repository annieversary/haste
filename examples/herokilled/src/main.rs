use haste::{
    entities::{self, Entity},
    fieldvalue::FieldValue,
    handler::HandlerVisitor,
    parser::{self, Context, Parser, Visitor},
    protos::{self, prost::Message},
    stringtables::StringTable,
};
use std::{fs::File, io::BufReader};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filepath = args.get(1);
    if filepath.is_none() {
        eprintln!("usage: herokilled <filepath>");
        std::process::exit(42);
    }

    let file = File::open(filepath.unwrap())?;
    let buf_reader = BufReader::new(file);

    let visitor = HandlerVisitor::with_state(State::default())
        .with(hero_killed)
        .with(chat_message);

    let mut parser = Parser::from_reader_with_visitor(buf_reader, visitor)?;
    parser.run_to_end()
}

#[derive(Default)]
struct State {
    deaths_at_tick: Vec<i32>,
}

fn hero_killed(state: &mut State, ctx: &Context, _msg: &protos::CCitadeLUserMsgHeroKilled) {
    state.deaths.push(ctx.tick());
}

fn chat_message(ctx: &Context, msg: &protos::CdotaUserMsgChatMessage) {
    println!("{:?}", msg);
}
