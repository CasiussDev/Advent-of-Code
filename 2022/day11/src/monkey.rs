use std::collections::VecDeque;

use crate::top2::Top2;

const WORRY_DECREASE_DIVIDER: u128 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorryDecEnabled {
    True,
    False,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorryOp {
    Add(u16),
    Mul(u16),
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DivisibleBy(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorryLevel(u128);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonkeyId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct InspectedCount(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Monkey {
    items: VecDeque<WorryLevel>,
    op: WorryOp,
    test: DivisibleBy,
    if_true: MonkeyId,
    if_false: MonkeyId,
    inspected_count: InspectedCount,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Troop {
    monkeys: Vec<Monkey>,
    thrown_items: Vec<(MonkeyId, WorryLevel)>,
}

impl MonkeyId {
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}

impl WorryLevel {
    pub fn new(value: u128) -> Self {
        Self(value)
    }

    fn decrease(&self) -> Self {
        Self(self.0 / WORRY_DECREASE_DIVIDER)
    }
}

impl WorryOp {
    fn apply_to(&self, item: WorryLevel) -> WorryLevel {
        println!("{}", item.0);
        match self {
            WorryOp::Add(sumand) => WorryLevel(item.0 + *sumand as u128),
            WorryOp::Mul(factor) => WorryLevel(item.0 * *factor as u128),
            WorryOp::Square => WorryLevel(item.0 * item.0),
        }
    }
}

impl DivisibleBy {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn is_true_for(&self, item: WorryLevel) -> bool {
        (item.0 % self.0 as u128) == 0
    }
}

impl Monkey {
    pub fn new(
        items: &[WorryLevel],
        op: WorryOp,
        test: DivisibleBy,
        if_true: MonkeyId,
        if_false: MonkeyId,
    ) -> Self {
        let mut monkey = Self {
            items: Default::default(),
            op,
            test,
            if_true,
            if_false,
            inspected_count: Default::default(),
        };
        monkey.items.extend(items.iter().cloned());
        monkey
    }

    pub fn turn(&mut self, throws: &mut Vec<(MonkeyId, WorryLevel)>, worry_dec: WorryDecEnabled) {
        let items_count = self.items.len();
        self.inspected_count = InspectedCount(items_count + self.inspected_count.0);
        for item in self.items.drain(..) {
            let inspected_item = if worry_dec == WorryDecEnabled::True {
                self.op.apply_to(item).decrease()
            } else {
                self.op.apply_to(item)
            };
            let destination_monkey = if self.test.is_true_for(inspected_item) {
                self.if_true
            } else {
                self.if_false
            };

            throws.push((destination_monkey, inspected_item));
        }
    }

    pub fn catch(&mut self, item: WorryLevel) {
        self.items.push_back(item);
    }
}

impl Troop {
    //pub fn new() -> Self {
    //    Self::default()
    //}

    pub fn new(monkeys: Vec<Monkey>) -> Self {
        Self {
            monkeys,
            thrown_items: Default::default(),
        }
    }

    //pub fn set_monkeys(&mut self, monkeys: Vec<Monkey>) {
    //    self.monkeys = monkeys;
    //}

    pub fn round(&mut self, worry_dec: WorryDecEnabled) {
        for monkey_id in 0..self.monkeys.len() {
            self.monkeys[monkey_id].turn(&mut self.thrown_items, worry_dec);

            for (monkey_id, item) in self.thrown_items.drain(..) {
                self.monkeys[monkey_id.0].catch(item);
            }
        }
    }

    pub fn top2_inspectors(&self) -> (Option<InspectedCount>, Option<InspectedCount>) {
        self.monkeys
            .iter()
            .map(|monkey| monkey.inspected_count)
            .top2()
    }
}

impl InspectedCount {
    pub fn get(&self) -> usize {
        self.0
    }
}
