use core::ops::{Add, Div, Mul, Sub};

use crate::{FloatChecker, NoisyFloat};

macro_rules! float_impls {
    ($ftp:ty) => {
        impl<C:FloatChecker<$ftp>> Add<NoisyFloat<$ftp, C>> for $ftp {
            type Output = NoisyFloat<$ftp, C>;
            #[inline]
            fn add(self, rhs: NoisyFloat<$ftp, C>) -> Self::Output {
                NoisyFloat::new(self) + rhs
            }
        }
        impl<C:FloatChecker<$ftp>> Sub<NoisyFloat<$ftp, C>> for $ftp {
            type Output = NoisyFloat<$ftp, C>;
            #[inline]
            fn sub(self, rhs: NoisyFloat<$ftp, C>) -> Self::Output {
                NoisyFloat::new(self) - rhs
            }
        }
        impl<C:FloatChecker<$ftp>> Mul<NoisyFloat<$ftp, C>> for $ftp {
            type Output = NoisyFloat<$ftp, C>;
            #[inline]
            fn mul(self, rhs: NoisyFloat<$ftp, C>) -> Self::Output {
                NoisyFloat::new(self) * rhs
            }
        }
        impl<C:FloatChecker<$ftp>> Div<NoisyFloat<$ftp, C>> for $ftp {
            type Output = NoisyFloat<$ftp, C>;
            #[inline]
            fn div(self, rhs: NoisyFloat<$ftp, C>) -> Self::Output {
                NoisyFloat::new(self) / rhs
            }
        }
    };
}
float_impls!(f32);
float_impls!(f64);