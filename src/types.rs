pub type BasisPoint = u32;

/// Full basis points, 10000 = 100%
pub const FULL_BASIS_POINT: BasisPoint = 10_000;

pub fn apply_basis_point(val: u128, bp: BasisPoint) -> u128 {
    val * bp as u128 / FULL_BASIS_POINT as u128
}
