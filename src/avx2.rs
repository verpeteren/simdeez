use super::*;
use overloads::*;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::mem;

pub struct Avx2;
impl Simd for Avx2 {
    type Vi32 = I32x8;
    type Vf32 = F32x8;
    type Vf64 = F64x4;

    const VF32_WIDTH: usize = 8;
    const VF64_WIDTH: usize = 4;
    const VI32_WIDTH: usize = 8;

    #[inline(always)]
    unsafe fn abs_ps(a: Self::Vf32) -> Self::Vf32 {
        let b = _mm256_set1_epi32(0x7fffffff);
        F32x8(_mm256_and_ps(a.0, _mm256_castsi256_ps(b)))
    }
    #[inline(always)]
    unsafe fn add_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_add_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn add_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_add_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn add_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_add_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn and_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_and_si256(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn andnot_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_andnot_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn andnot_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_andnot_si256(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn blendv_epi32(a: Self::Vi32, b: Self::Vi32, mask: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_castps_si256(_mm256_blendv_ps(
            _mm256_castsi256_ps(a.0),
            _mm256_castsi256_ps(b.0),
            _mm256_castsi256_ps(mask.0),
        )))
    }
    #[inline(always)]
    unsafe fn blendv_ps(a: Self::Vf32, b: Self::Vf32, mask: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_blendv_ps(a.0, b.0, mask.0))
    }
    #[inline(always)]
    unsafe fn castps_si(a: Self::Vf32) -> Self::Vi32 {
        I32x8(_mm256_castps_si256(a.0))
    }
    #[inline(always)]
    unsafe fn castsi_ps(a: Self::Vi32) -> Self::Vf32 {
        F32x8(_mm256_castsi256_ps(a.0))
    }
    #[inline(always)]
    unsafe fn cmpeq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_cmpeq_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpge_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_cmp_ps(a.0, b.0, _CMP_GE_OQ))
    }
    #[inline(always)]
    unsafe fn cmpgt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_cmpgt_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpgt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_cmp_ps(a.0, b.0, _CMP_GT_OQ))
    }
    #[inline(always)]
    unsafe fn cmplt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_cmp_ps(a.0, b.0, _CMP_LT_OQ))
    }
    #[inline(always)]
    unsafe fn cvtepi32_ps(a: Self::Vi32) -> Self::Vf32 {
        F32x8(_mm256_cvtepi32_ps(a.0))
    }
    #[inline(always)]
    unsafe fn cvtps_epi32(a: Self::Vf32) -> Self::Vi32 {
        I32x8(_mm256_cvtps_epi32(a.0))
    }
    #[inline(always)]
    unsafe fn ceil_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_ceil_ps(a.0))
    }
    #[inline(always)]
    unsafe fn floor_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_floor_ps(a.0))
    }
    #[inline(always)]
    unsafe fn fastfloor_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_floor_ps(a.0))
    }
    #[inline(always)]
    unsafe fn fmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_fmadd_ps(a.0, b.0, c.0))
    }
    #[inline(always)]
    unsafe fn fnmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_fnmadd_ps(a.0, b.0, c.0))
    }
    #[inline(always)]
    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_i32gather_epi32(&arr[0] as *const i32, index.0, 4))
    }
    #[inline(always)]
    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32 {
        F32x8(_mm256_i32gather_ps(&arr[0] as *const f32, index.0, 4))
    }
    #[inline(always)]
    unsafe fn loadu_ps(a: &f32) -> Self::Vf32 {
        F32x8(_mm256_loadu_ps(a as *const f32))
    }
    #[inline(always)]
    unsafe fn loadu_pd(a: &f64) -> Self::Vf64 {
        F64x4(_mm256_loadu_pd(a as *const f64))
    }
    #[inline(always)]
    unsafe fn loadu_si(a: &i32) -> Self::Vi32 {
        let m = mem::transmute::<&i32, &__m256i>(a);
        I32x8(_mm256_loadu_si256(m))
    }
    #[inline(always)]
    unsafe fn storeu_ps(a: &mut f32, b: Self::Vf32) {
        _mm256_storeu_ps(a as *mut f32, b.0);
    }
    #[inline(always)]
    unsafe fn max_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_max_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn min_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_min_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn mul_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_mul_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn mul_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_mul_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn div_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_div_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn mullo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_mullo_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn or_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_or_si256(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn round_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_round_ps(
            a.0,
            _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC,
        ))
    }
    #[inline(always)]
    unsafe fn set1_epi32(a: i32) -> Self::Vi32 {
        I32x8(_mm256_set1_epi32(a))
    }
    #[inline(always)]
    unsafe fn set1_ps(a: f32) -> Self::Vf32 {
        F32x8(_mm256_set1_ps(a))
    }
    #[inline(always)]
    unsafe fn set1_pd(a: f64) -> Self::Vf64 {
        F64x4(_mm256_set1_pd(a))
    }
    #[inline(always)]
    unsafe fn setzero_pd() -> Self::Vf64 {
        F64x4(_mm256_setzero_pd())
    }
    #[inline(always)]
    unsafe fn setzero_ps() -> Self::Vf32 {
        F32x8(_mm256_setzero_ps())
    }
    #[inline(always)]
    unsafe fn setzero_si() -> Self::Vi32 {
        I32x8(_mm256_setzero_si256())
    }
    #[inline(always)]
    unsafe fn srai_epi32(a: Self::Vi32, b: i32) -> Self::Vi32 {
        I32x8(_mm256_srai_epi32(a.0, b))
    }
    #[inline(always)]
    unsafe fn sub_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_sub_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn sub_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_sub_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn sqrt_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_sqrt_ps(a.0))
    }
    #[inline(always)]
    unsafe fn rsqrt_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_rsqrt_ps(a.0))
    }
    #[inline(always)]
    unsafe fn xor_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_xor_si256(a.0, b.0))
    }
}
