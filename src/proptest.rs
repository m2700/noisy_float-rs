use core::{fmt::Debug, marker::PhantomData};

use num_traits::Float;
use proptest::{
    arbitrary::Arbitrary,
    num::f32,
    num::f64,
    strategy::{NewTree, Strategy, ValueTree},
    test_runner::TestRunner,
};

use crate::{FloatChecker, NoisyFloat, checkers::{FiniteChecker, NumChecker}};

impl<F: Float, C: FloatChecker<F>> Arbitrary for NoisyFloat<F, C>
where
    F: Debug,
    Any<F, C>: Strategy<Value = Self>,
{
    type Parameters = ();
    type Strategy = Any<F, C>;
    #[inline]
    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        Any(PhantomData)
    }
}

pub struct Any<F, C>(PhantomData<(F, C)>);
impl<F, C> Debug for Any<F, C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Any")
    }
}

pub struct Tree<T, C>(T, PhantomData<C>);

impl<T, C> ValueTree for Tree<T, C>
where
    T: ValueTree,
    C: FloatChecker<T::Value>,
    T::Value: Float + Debug,
{
    type Value = NoisyFloat<T::Value, C>;
    #[inline]
    fn current(&self) -> Self::Value {
        Self::Value::unchecked_new_generic(self.0.current())
    }
    #[inline]
    fn simplify(&mut self) -> bool {
        self.0.simplify()
    }
    #[inline]
    fn complicate(&mut self) -> bool {
        self.0.complicate()
    }
}

macro_rules! float_any_strategy_impls {
    ($ftp:ident) => {
        impl Strategy for Any<$ftp, NumChecker> {
            type Value = NoisyFloat<$ftp, NumChecker>;
            type Tree = Tree<$ftp::BinarySearch, NumChecker>;
            #[inline]
            fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
                ($ftp::INFINITE
                    | $ftp::NEGATIVE
                    | $ftp::POSITIVE
                    | $ftp::NORMAL
                    | $ftp::SUBNORMAL
                    | $ftp::ZERO)
                    .new_tree(runner)
                    .map(|t| Tree(t, PhantomData))
            }
        }
        impl Strategy for Any<$ftp, FiniteChecker> {
            type Value = NoisyFloat<$ftp, FiniteChecker>;
            type Tree = Tree<$ftp::BinarySearch, FiniteChecker>;
            #[inline]
            fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
                ($ftp::NEGATIVE
                    | $ftp::POSITIVE
                    | $ftp::NORMAL
                    | $ftp::SUBNORMAL
                    | $ftp::ZERO)
                    .new_tree(runner)
                    .map(|t| Tree(t, PhantomData))
            }
        }
    };
}
float_any_strategy_impls!(f32);
float_any_strategy_impls!(f64);
