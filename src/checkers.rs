// Copyright 2016-2021 Matthew D. Michelotti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Standard implementations of `FloatChecker`.

use core::convert::TryFrom;

use crate::{FloatChecker, NoisyFloat};
use num_traits::Float;

/// A `FloatChecker` that considers all values valid except NaN.
///
/// This checks that the value is a "number", i.e. it is not "not-a-number".
///
/// The `assert` method is implemented using `debug_assert!`.
pub struct NumChecker;

impl<F: Float> FloatChecker<F> for NumChecker {
    #[track_caller]
    #[inline]
    fn assert(value: F) {
        debug_assert!(Self::check(value), "unexpected NaN");
    }

    #[inline]
    fn check(value: F) -> bool {
        !value.is_nan()
    }
}

/// A `FloatChecker` that considers all values valid except NaN and +/- Infinity.
///
/// The `assert` method is implemented using `debug_assert!`.
pub struct FiniteChecker;

impl<F: Float> FloatChecker<F> for FiniteChecker {
    #[track_caller]
    #[inline]
    fn assert(value: F) {
        debug_assert!(Self::check(value), "unexpected NaN or infinity");
    }

    #[inline]
    fn check(value: F) -> bool {
        value.is_finite()
    }
}

impl<F: Float> From<NoisyFloat<F, FiniteChecker>> for NoisyFloat<F, NumChecker> {
    #[inline]
    fn from(value: NoisyFloat<F, FiniteChecker>) -> Self {
        Self::unchecked_new_generic(value.raw())
    }
}

impl<F: Float> TryFrom<NoisyFloat<F, NumChecker>> for NoisyFloat<F, FiniteChecker> {
    type Error = &'static str;
    #[inline]
    fn try_from(f: NoisyFloat<F, NumChecker>) -> Result<Self, Self::Error> {
        Self::try_new(f.value).ok_or("illegal value")
    }
}