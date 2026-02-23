mod scalar;

pub trait AlgebraicOps {
    fn algebraic_add(self, rhs: Self) -> Self;
    fn algebraic_sub(self, rhs: Self) -> Self;
    fn algebraic_mul(self, rhs: Self) -> Self;
    fn algebraic_div(self, rhs: Self) -> Self;
    fn algebraic_rem(self, rhs: Self) -> Self;
}
