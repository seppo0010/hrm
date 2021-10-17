use crate::context::{Context, Tile};

#[derive(Clone, Copy, Debug)]
pub enum Expression<'a> {
    Inbox,
    Outbox,
    CopyFrom(usize),
    CopyTo(usize),
    Add(usize),
    Sub(usize),
    Incr(usize),
    Decr(usize),
    Label(&'a str),
    Jump(&'a str),
    JumpIfZero(&'a str),
    JumpIfNegative(&'a str),
}

impl<'a> Expression<'a>  {
    fn run(&self, runner: &mut Runner) -> bool {
        match *self {
            Expression::Inbox =>{
                runner.context.inbox();
                runner.context.hand.is_none()
            },
            Expression::Outbox => { runner.context.outbox(); false },
            Expression::CopyFrom(s) => { runner.context.copyfrom(s); false },
            Expression::CopyTo(s) => { runner.context.copyto(s); false },
            Expression::Incr(s) => { runner.context.incr(s); false },
            Expression::Decr(s) => { runner.context.decr(s); false },
            Expression::Add(s) => { runner.context.add(s); false },
            Expression::Sub(s) => { runner.context.sub(s); false },
            Expression::Label(_) => { false },
            Expression::Jump(s) => { runner.jump(s); false },
            Expression::JumpIfZero(s) => {
                if runner.context.hand.clone().expect("jump if with empty hands").num() == 0 {
                    runner.jump(s);
                }
                false
            }
            Expression::JumpIfNegative(s) => {
                if runner.context.hand.clone().expect("jump if with empty hands").num() < 0 {
                    runner.jump(s);
                }
                false
            }
        }
    }
}

pub struct Runner<'a> {
    statements: Vec<Expression<'a>>,
    context: Context,
    pos: usize,
}

impl<'a> Runner<'a> {
    pub fn jump(&mut self, label: &str) {
        self.pos = self.statements.iter().position(|&st| match st {
            Expression::Label(name) => name == label,
            _ => false,
        }).expect("Jump to unexisting label");
    }

    pub fn run(mut self) -> Vec<Tile> {
        while self.pos < self.statements.len() {
            if self.statements[self.pos].clone().run(&mut self) {
                break
            }
            self.pos += 1;
        }
        self.context.outbox_line
    }
}

#[test]
fn test_inbox_outbox() {
    assert_eq!(Runner {
        context: Context::new(&vec![
            Tile::Num(3),
            Tile::Num(31),
        ], 10),
        pos: 0,
        statements: vec![
            Expression::Inbox,
            Expression::Outbox,
            Expression::Inbox,
            Expression::Outbox,
        ],
    }.run(),
        vec![
            Tile::Num(3),
            Tile::Num(31),
        ]
    )
}

#[test]
fn test_copyfrom_copyto() {
    assert_eq!(Runner {
        context: Context::new(&vec![
            Tile::Num(3),
            Tile::Num(31),
        ], 10),
        pos: 0,
        statements: vec![
            Expression::Inbox,
            Expression::CopyTo(0),
            Expression::Inbox,
            Expression::Outbox,
            Expression::CopyFrom(0),
            Expression::Outbox,
        ],
    }.run(),
        vec![
            Tile::Num(31),
            Tile::Num(3),
        ]
    )
}

#[test]
fn test_incr_decr() {
    assert_eq!(Runner {
        context: Context::new(&vec![
            Tile::Num(3),
            Tile::Num(31),
        ], 10),
        pos: 0,
        statements: vec![
            Expression::Inbox,
            Expression::CopyTo(0),
            Expression::Incr(0),
            Expression::Outbox,
            Expression::Inbox,
            Expression::CopyTo(0),
            Expression::Decr(0),
            Expression::Outbox,
        ],
    }.run(),
        vec![
            Tile::Num(4),
            Tile::Num(30),
        ]
    )
}

#[test]
fn test_jump() {
    assert_eq!(Runner {
        context: Context::new(&vec![
            Tile::Num(3),
            Tile::Num(31),
        ], 10),
        pos: 0,
        statements: vec![
            Expression::Inbox,
            Expression::Jump("skip"),
            Expression::Outbox,
            Expression::Label("skip"),
            Expression::Outbox,
        ],
    }.run(),
        vec![
            Tile::Num(3),
        ]
    )
}

#[test]
fn test_empty_inbox_ends() {
    assert_eq!(Runner {
        context: Context::new(&vec![
            Tile::Num(3),
            Tile::Num(31),
        ], 10),
        pos: 0,
        statements: vec![
            Expression::Label("start"),
            Expression::Inbox,
            Expression::Outbox,
            Expression::Jump("start"),
        ],
    }.run(),
        vec![
            Tile::Num(3),
            Tile::Num(31),
        ]
    )
}

#[test]
fn test_jumpifnegative() {
    assert_eq!(Runner {
        context: Context::new(&vec![
            Tile::Num(-31),
            Tile::Num(3),
            Tile::Num(-4),
        ], 10),
        pos: 0,
        statements: vec![
            Expression::Label("start"),
            Expression::Inbox,
            Expression::JumpIfNegative("start"),
            Expression::Outbox,
            Expression::Jump("start"),
        ],
    }.run(),
        vec![
            Tile::Num(3),
        ]
    )
}

#[test]
fn test_jumpifzero() {
    assert_eq!(Runner {
        context: Context::new(&vec![
            Tile::Num(-31),
            Tile::Num(0),
            Tile::Num(4),
        ], 10),
        pos: 0,
        statements: vec![
            Expression::Label("start"),
            Expression::Inbox,
            Expression::JumpIfZero("start"),
            Expression::Outbox,
            Expression::Jump("start"),
        ],
    }.run(),
        vec![
            Tile::Num(-31),
            Tile::Num(4),
        ]
    )
}