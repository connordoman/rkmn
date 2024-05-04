use self::{
    attack_data::AttackData, condition_data::ConditionData, growth_data::GrowthData,
    misc_data::MiscData,
};

pub mod attack_data;
pub mod condition_data;
pub mod growth_data;
pub mod misc_data;
pub mod species_data;
pub mod type_data;

#[derive(Debug, Clone, Copy)]
pub struct RkmnData {
    attack_data: AttackData,
    condition_data: ConditionData,
    growth_data: GrowthData,
    misc_data: MiscData,
}

impl RkmnData {
    pub fn new() -> Self {
        Self {
            attack_data: AttackData::new(),
            condition_data: ConditionData::new(),
            growth_data: GrowthData::new(),
            misc_data: MiscData::new(),
        }
    }
}

pub fn compute_substruct_order(personality_value: u32) -> [u8; 4] {
    let rem = personality_value % 24;
    let mut order: [u8; 4] = [0; 4];
    let order_len = order.len();
    heap_permutation(&mut order, order_len, rem);
    order
}

fn heap_permutation(a: &mut [u8], size: usize, n: u32) {
    if size == 1 {
        println!("{:?}", a);
        return;
    }

    for i in 0..size {
        heap_permutation(a, size - 1, n);

        if size % 2 == 1 {
            a.swap(0, size - 1);
        } else {
            a.swap(i, size - 1);
        }
    }
}
