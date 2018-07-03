use super::*;
// -- Bitwise And
impl BitAnd for I32x1 {
    type Output = I32x1;

    fn bitand(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0 & rhs.0)
    }
}
impl BitAnd for I32x4 {
    type Output = I32x4;

    fn bitand(self, rhs: I32x4) -> I32x4 {
        I32x4(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}
impl BitAnd for I32x4_41 {
    type Output = I32x4_41;

    fn bitand(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}
impl BitAnd for I32x8 {
    type Output = I32x8;

    fn bitand(self, rhs: I32x8) -> I32x8 {
        I32x8(unsafe { _mm256_and_si256(self.0, rhs.0) })
    }
}
impl BitAnd for F32x1 {
    type Output = F32x1;

    fn bitand(self, rhs: F32x1) -> F32x1 {
        unsafe {
            let self_i = mem::transmute::<F32x1, i32>(self);
            let rhs_i = mem::transmute::<F32x1, i32>(rhs);
            mem::transmute::<i32, F32x1>(self_i & rhs_i)
        }
    }
}
impl BitAnd for F32x4 {
    type Output = F32x4;

    fn bitand(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_and_ps(self.0, rhs.0) })
    }
}
impl BitAnd for F32x8 {
    type Output = F32x8;

    fn bitand(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_and_ps(self.0, rhs.0) })
    }
}
