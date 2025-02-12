use crate::field::{
    element::FieldElement,
    fields::montgomery_backed_prime_fields::{
        IsMontgomeryConfiguration, MontgomeryBackendPrimeField,
    },
};
use crate::unsigned_integer::element::U384;

pub const BLS12377_PRIME_FIELD_ORDER: U384 = U384::from("1ae3a4617c510eac63b05c06ca1493b1a22d9f300f5138f1ef3622fba094800170b5d44300000008508c00000000001");

// FPBLS12377
#[derive(Clone, Debug)]
pub struct BLS12377FieldConfig;
impl IsMontgomeryConfiguration<6> for BLS12377FieldConfig {
    const MODULUS: U384 = BLS12377_PRIME_FIELD_ORDER;
}

pub type BLS12377PrimeField = MontgomeryBackendPrimeField<BLS12377FieldConfig, 6>;

impl FieldElement<BLS12377PrimeField> {
    pub fn new_base(a_hex: &str) -> Self {
        Self::new(U384::from(a_hex))
    }
}
