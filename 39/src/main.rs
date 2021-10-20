use hrm::{Runner, Context, Expression, Tile};

// "Each number in the INBOX is an address of a tile on the floor. Send to the OUTBOX the coordinates of that tile, column first, row second.\n\nFor example, an address of 6 has coordinates 2, 1. You may ask your boss for more examples.",
// cols:  0 1 2 3
// row 0: 0 1 2 3
// row 1: 4 5 6 7
// row 2: 8 9 a b
// row 3: c d e f
// output is input % 4, floor(input / 4)
const FOUR: usize = 15;
const ZERO: usize = 14;
const COUNTER: usize = 13;
const VALUE: usize = 12;
fn statements<'a>() -> Vec<Expression<'a>> {
    vec![
        Expression::Label("new letter"),
        Expression::CopyFrom(ZERO),
        Expression::CopyTo(COUNTER),
        Expression::Inbox,
        Expression::CopyTo(VALUE),
        Expression::Label("nextcol"),
        Expression::CopyFrom(VALUE),
        Expression::Sub(FOUR),
        Expression::JumpIfNegative("outcol"),
        Expression::CopyTo(VALUE),
        Expression::Incr(COUNTER),
        Expression::Jump("nextcol"),
        Expression::Label("outcol"),
        Expression::CopyFrom(VALUE),
        Expression::Outbox,
        Expression::CopyFrom(COUNTER),
        Expression::Outbox,
        Expression::Jump("new letter"),
    ]
}

fn context_with_inbox(inbox: Vec<Tile>) -> Context {
    let mut context = Context::new(&inbox, 16);
    context.memory[ZERO] = Some(Tile::Num(0));
    context.memory[FOUR] = Some(Tile::Num(4));
    context
}

fn main() {
    assert_eq!(
        Runner {
            context: context_with_inbox(vec![
                Tile::Num(0),
                Tile::Num(1),
                Tile::Num(2),
                Tile::Num(3),
                Tile::Num(4),
                Tile::Num(5),
                Tile::Num(6),
            ]),
            pos: 0,
            statements: statements(),
        }.run_debug(),
        vec![
            Tile::Num(0),
            Tile::Num(0),
            Tile::Num(1),
            Tile::Num(0),
            Tile::Num(2),
            Tile::Num(0),
            Tile::Num(3),
            Tile::Num(0),
            Tile::Num(0),
            Tile::Num(1),
            Tile::Num(1),
            Tile::Num(1),
            Tile::Num(2),
            Tile::Num(1),
        ],
    )
}
