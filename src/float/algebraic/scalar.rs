use super::AlgebraicOps;

impl AlgebraicOps for f32 {
    fn algebraic_add(self, rhs: Self) -> Self {
        self.algebraic_add(rhs)
    }
    fn algebraic_sub(self, rhs: Self) -> Self {
        self.algebraic_sub(rhs)
    }
    fn algebraic_mul(self, rhs: Self) -> Self {
        self.algebraic_mul(rhs)
    }
    fn algebraic_div(self, rhs: Self) -> Self {
        self.algebraic_div(rhs)
    }
    fn algebraic_rem(self, rhs: Self) -> Self {
        self.algebraic_rem(rhs)
    }
}
