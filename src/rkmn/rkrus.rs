use rand::*;

pub enum RkrusStrain {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Clone, Copy)]
pub struct Rkrus {
    strain_x: u8,
    days: u8,
}

impl Rkrus {
    pub fn new() -> Rkrus {
        let mut rus_type: Rkrus = Rkrus {
            strain_x: 0,
            days: 0,
        };
        rus_type.strain_x = rus_type.random_rkrs_strain();
        rus_type.days = rus_type.calculate_days();
        Rkrus {
            strain_x: rus_type.strain_x,
            days: rus_type.days,
        }
    }

    fn random_rkrs_strain(&self) -> u8 {
        let random_strain = thread_rng().gen_range(0..256);
        if random_strain <= 31 {
            // 31/255 chance
            return thread_rng().gen_range(1..8);
        } else if random_strain == 255 {
            // 1/255 chance
            return thread_rng().gen_range(8..16);
        }
        return 0;
    }

    pub fn calculate_days(&self) -> u8 {
        let masked = &self.strain_x & 0b00001111;
        let days = masked & 4 + 1;
        days
    }

    pub fn get_rkrs_strain(&self) -> RkrusStrain {
        match self.strain_x {
            0 => RkrusStrain::A,
            1 => RkrusStrain::B,
            2 => RkrusStrain::C,
            3 => RkrusStrain::D,
            _ => panic!("Invalid strain"),
        }
    }
}
