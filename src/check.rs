use std::marker::PhantomData;

use crate::cmberr::{
    CombineErrorBuilder,
    CombineError
};

pub trait Check<T, Pre> 
    where Self:Sized
{
    type State;
    type Error;

    fn check(self, value: CheckState<T, Pre>)
        -> CheckOutcome<T, Self::State, Self::Error>;

    fn and<B, C>(self, b: B) -> And<Self, B, C>
        where
            B: Check<T, Self::State>,
            C: CombineErrorBuilder<Self::Error, B::Error>
    {
        And { a: self, b, _combine:PhantomData }
    }

    fn or<B, C>(self, b: B) -> Or<Self, B, C>
        where 
            B: Check<T, Self::State>,
            C: CombineErrorBuilder<Self::Error, B::Error>
    {
        Or { a: self, b, _combine: PhantomData }
    }
}

pub enum CheckOutcome<T, State, E> {
    Passed(CheckState<T, State>),
    Failed{
        state: CheckState<T, State>,
        err: E
    },
}

pub struct CheckState<T: Sized, S> 
    where Self: Sized 
{
    pub value: T,
    _state: PhantomData<S>
}

impl<T, S> CheckState<T, S> {
    pub fn new(value: T) -> Self {
        Self { value, _state: PhantomData }
    }
}

pub struct And<A, B, C> {
    a: A,
    b: B,
    _combine: PhantomData<C>
}

impl<T, Pre, A, B, C> Check<T, Pre> for And<A, B, C>
where
    A: Check<T, Pre>,
    B: Check<T, A::State>,
    C: CombineErrorBuilder<A::Error, B::Error>,
{
    type State = B::State;
    type Error =  <C::Combiner as CombineError<A::Error, B::Error>>::Out;

    fn check(self, value: CheckState<T, Pre>)
        -> CheckOutcome<T, Self::State, Self::Error>
    {
        let mut combine = <C as CombineErrorBuilder<A::Error, B::Error>>::build();

        match self.a.check(value) {
            CheckOutcome::Passed(v) => {
                match self.b.check(v){
                    CheckOutcome::Passed(vv) => {
                        // success A and success B
                        CheckOutcome::Passed(vv)
                    }
                    CheckOutcome::Failed{state, err} => {
                        // success A and failed B
                        combine.right(err);
                        CheckOutcome::Failed{
                            state: CheckState { value: state.value, _state: PhantomData },
                            err: combine.finish()
                        }
                    }
                }
            }
            CheckOutcome::Failed{state, err} => {
                // failed A
                combine.left(err);
                CheckOutcome::Failed{
                    state: CheckState { value: state.value, _state: PhantomData },
                    err: combine.finish()
                }
            }
        }
    }
}

pub struct Or<A, B, C> {
    a: A,
    b: B,
    _combine: PhantomData<C>
}

impl<T, Pre, A, B, C> Check<T, Pre> for Or<A, B, C>
where
    A: Check<T, Pre>,
    B: Check<T, A::State>,
    C: CombineErrorBuilder<A::Error, B::Error>,
{
    type State = B::State;
    type Error = C::Out;

    fn check(self, value: CheckState<T, Pre>)
        -> CheckOutcome<T, Self::State, Self::Error>
    {
        let mut combine = <C as CombineErrorBuilder<A::Error, B::Error>>::build();

        match self.a.check(value) {
            CheckOutcome::Passed(v) => {
                // success A
                match self.b.check(v) {
                    CheckOutcome::Passed(vv) => {
                        // success A and success B
                        CheckOutcome::Passed(vv)
                    }
                    CheckOutcome::Failed{state, err} => {
                        // success A and failed B
                        combine.right(err);
                        CheckOutcome::Failed{
                            state: CheckState { value: state.value, _state: PhantomData },
                            err: combine.finish()
                        }
                    }
                }
            }
            CheckOutcome::Failed{state, err} => {
                // failed A
                combine.left(err);
                match self.b.check(state) {
                    CheckOutcome::Passed(vv) => {
                        // failed A and success B
                        CheckOutcome::Failed{
                            state: CheckState { value: vv.value, _state: PhantomData },
                            err: combine.finish()
                        }
                    }
                    CheckOutcome::Failed{state, err} => {
                        // failed A and failed B
                        combine.right(err);
                        CheckOutcome::Failed{
                            state: CheckState { value: state.value, _state: PhantomData },
                            err: combine.finish()
                        }
                    }
                }
            }
        }
    }
}

impl<T, Pre, State, F, E> Check<T, Pre> for F
where
    F: Fn(CheckState<T, Pre>) -> CheckOutcome<T, State, E>,
{
    type State = State;
    type Error = E;

    fn check(self, value: CheckState<T, Pre>)
        -> CheckOutcome<T, Self::State, Self::Error>
    {
        self(value)
    }
}

