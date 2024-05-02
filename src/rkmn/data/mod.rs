pub mod attack_data;
pub mod condition_data;
pub mod growth_data;
pub mod misc_data;
pub mod type_data;

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
