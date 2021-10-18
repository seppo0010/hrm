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
    CopyFromPointer(usize),
    CopyToPointer(usize),
    AddPointer(usize),
    SubPointer(usize),
    IncrPointer(usize),
    DecrPointer(usize),
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
                let hand = runner.context.hand.as_ref();
                hand.is_none()
            },
            Expression::Outbox => { runner.context.outbox(); false },
            Expression::CopyFrom(s) => { runner.context.copyfrom(s); false },
            Expression::CopyTo(s) => { runner.context.copyto(s); false },
            Expression::Incr(s) => { runner.context.incr(s); false },
            Expression::Decr(s) => { runner.context.decr(s); false },
            Expression::Add(s) => { runner.context.add(s); false },
            Expression::Sub(s) => { runner.context.sub(s); false },
            Expression::CopyFromPointer(s) => { runner.context.copyfromp(s); false },
            Expression::CopyToPointer(s) => { runner.context.copytop(s); false },
            Expression::IncrPointer(s) => { runner.context.incrp(s); false },
            Expression::DecrPointer(s) => { runner.context.decrp(s); false },
            Expression::AddPointer(s) => { runner.context.addp(s); false },
            Expression::SubPointer(s) => { runner.context.subp(s); false },
            Expression::Label(_) => { false },
            Expression::Jump(s) => { runner.jump(s); false },
            Expression::JumpIfZero(s) => {
                let hand = runner.context.hand.as_ref();
                let tile = hand.clone().expect("jump if with empty hands");
                if tile.is_num() && tile.num() == 0 {
                    runner.jump(s);
                }
                false
            }
            Expression::JumpIfNegative(s) => {
                let hand = runner.context.hand.as_ref();
                let tile = hand.clone().expect("jump if with empty hands");
                if tile.is_num() && tile.num() < 0 {
                    runner.jump(s);
                }
                false
            }
        }
    }
}

pub struct Runner<'a> {
    pub statements: Vec<Expression<'a>>,
    pub context: Context,
    pub pos: usize,
}

impl<'a> Runner<'a> {
    pub fn jump(&mut self, label: &str) {
        self.pos = self.statements.iter().position(|&st| match st {
            Expression::Label(name) => name == label,
            _ => false,
        }).expect("Jump to unexisting label");
    }

    fn do_run(mut self, debug: bool) -> Vec<Tile> {
        while self.pos < self.statements.len() {
            if debug {
                // TODO: use log lib
                println!("running statement {:?}, hand has: {:?}", self.statements[self.pos], self.context.hand);
            }
            if self.statements[self.pos].clone().run(&mut self) {
                break
            }
            self.pos += 1;
        }
        self.context.outbox_line
    }

    pub fn run_debug(self) -> Vec<Tile> {
        self.do_run(true)
    }

    pub fn run(self) -> Vec<Tile> {
        self.do_run(false)
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
