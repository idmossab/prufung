#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl BloodType {
    pub fn can_receive_from(self, other: Self) -> bool {
        self.donors().contains(&other)
    }

    pub fn donors(self) -> Vec<Self> {
        Self::all_types().into_iter().filter(|b| Self::comp(b,&self)).collect()
    }

    pub fn recipients(self) -> Vec<Self> {
        Self::all_types().into_iter().filter(|b| Self::comp(&self,b)).collect()
    }

    pub fn all_types() -> Vec<Self> {
        use crate::Antigen::*;
        use crate::RhFactor::*;
        vec![
            Self{antigen:AB,rh_factor:Positive},
            Self{antigen:AB,rh_factor:Negative},

            Self{antigen:A,rh_factor:Positive},
            Self{antigen:A,rh_factor:Negative},

            Self{antigen:B,rh_factor:Positive},
            Self{antigen:B,rh_factor:Negative},

            Self{antigen:O,rh_factor:Positive},
            Self{antigen:O,rh_factor:Negative},
        ]
    }

    pub fn comp(d:&BloodType,r:&BloodType) -> bool {
        use crate::Antigen::*;
        use crate::RhFactor::*;
        if d.rh_factor==Positive && r.rh_factor==Negative{
            return false
        }
        match d.antigen{
            O=>true,
            A=>r.antigen==A ||r.antigen==AB,
            B=>r.antigen==B ||r.antigen==AB,
            AB=>r.antigen==AB
        }
    }
}