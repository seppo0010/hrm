use hrm::{Runner, Context, Expression, Tile};

fn statements<'a>() -> Vec<Expression<'a>> {
    vec![
        Expression::Jump("start"),
        Expression::Label("out"),
        Expression::Decr(13),
        Expression::JumpIfNegative("start"),
        Expression::CopyFromPointer(13),
        Expression::Outbox,
        Expression::Jump("out"),
        Expression::Label("start"),
        Expression::CopyFrom(14),
        Expression::CopyTo(13),
        Expression::Label("next"),
        Expression::Inbox,
        Expression::JumpIfZero("out"),
        Expression::CopyToPointer(13),
        Expression::Incr(13),
        Expression::Jump("next"),
    ]
}

fn context_with_inbox(inbox: Vec<Tile>) -> Context {
    let mut context = Context::new(&inbox, 15);
    context.memory[14] = Some(Tile::Num(0));
    context
}

fn main() {
    assert_eq!(
        Runner {
            context: context_with_inbox(vec![
                Tile::Char('A'),
                Tile::Char('B'),
                Tile::Char('C'),
                Tile::Num(0),
                Tile::Char('D'),
                Tile::Char('E'),
                Tile::Char('F'),
                Tile::Num(0),
            ]),
            pos: 0,
            statements: statements(),
        }.run(),
        vec![
            Tile::Char('C'),
            Tile::Char('B'),
            Tile::Char('A'),
            Tile::Char('F'),
            Tile::Char('E'),
            Tile::Char('D'),
        ],
    )
}
