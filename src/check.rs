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
    value: T,
    _state: PhantomData<S>
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

// -------------------------------------------------

#[cfg(test)]
mod tests_n {
    use super::*;
    use crate::cmberr::CustomCombine;

    struct Checked;
    struct Unchecked;
    struct ErrState<CheckStartsWithHello, CheckMin3, CheckEndsWithWorld, CheckIncludesAbc> {
        _check_starts_with_hello: PhantomData<CheckStartsWithHello>,
        _check_min3: PhantomData<CheckMin3>,
        _check_ends_with_world: PhantomData<CheckEndsWithWorld>,
        _check_includes_abc: PhantomData<CheckIncludesAbc>
    }

    #[derive(Debug)]
    enum ValidateErr {
        CheckStartsWithHelloErr,
        CheckMin6Err,
        CheckEndsWithWorldErr,
        CheckIncludesAbcErr,
    }

    fn check_starts_with_hello(
        data: CheckState<&str, ErrState<Unchecked, Unchecked, Unchecked, Unchecked>>) 
    -> 
    CheckOutcome<&str, ErrState<Checked, Unchecked, Unchecked, Unchecked>, ValidateErr>
    {
        if data.value.starts_with("hello") {
            CheckOutcome::Passed(
                CheckState { value: data.value, _state: PhantomData }
            )
        } else {
            CheckOutcome::Failed{
                state: CheckState { value: data.value, _state: PhantomData },
                err: ValidateErr::CheckStartsWithHelloErr
            }
        }
    }

    fn check_min6(
        data: CheckState<&str, ErrState<Checked, Unchecked, Unchecked, Unchecked>>) 
    -> 
    CheckOutcome<&str, ErrState<Checked, Checked, Unchecked, Unchecked>, ValidateErr>
    {
        if 6 < data.value.len() {
            CheckOutcome::Passed(
                CheckState { value: data.value, _state: PhantomData }
            )
        }
        else {
            CheckOutcome::Failed{
                state: CheckState { value: data.value, _state: PhantomData },
                err: ValidateErr::CheckMin6Err
            }
        }
    }

    fn check_ends_with_world(
        data: CheckState<&str, ErrState<Checked, Checked, Unchecked, Unchecked>>) 
    -> 
    CheckOutcome<&str, ErrState<Checked, Checked, Checked, Unchecked>, ValidateErr>
    {
        if data.value.ends_with("world") {
            CheckOutcome::Passed(
                CheckState { value: data.value, _state: PhantomData }
            )
        } else {
            CheckOutcome::Failed{
                state: CheckState { value: data.value, _state: PhantomData },
                err: ValidateErr::CheckEndsWithWorldErr
            }
        }
    }

    fn check_includes_abc(
        data: CheckState<&str, ErrState<Checked, Checked, Checked, Unchecked>>) 
    -> 
    CheckOutcome<&str, ErrState<Checked, Checked, Checked, Checked>, ValidateErr>
    {
        if let Some(_) = data.value.find("abc") {
            CheckOutcome::Passed(
                CheckState { value: data.value, _state: PhantomData }
            )
        } else {
            CheckOutcome::Failed{
                state: CheckState { value: data.value, _state: PhantomData },
                err: ValidateErr::CheckIncludesAbcErr
            }
        }
    }

    #[test]
    fn n_works00() {
        //let s = "hello abc world";
        let s = " abc world";
        let s = "hello world";

        let checker =
            (check_starts_with_hello
            .or::<_, CustomCombine<ValidateErr>>(check_min6))
            .or::<_, CustomCombine<ValidateErr>>(check_ends_with_world)
            .or::<_, CustomCombine<ValidateErr>>(check_includes_abc);

            // .and::<_, DefaultCombine>(check_min6);

        let r = checker.check(
            CheckState { value: s, _state: PhantomData }
        );

        match r {
            CheckOutcome::Passed(_v) => {
                println!("Passed!");
            }
            CheckOutcome::Failed{state:_, err} => {
                println!("Failed because");
                println!("{:?}", err)
            }
        }
    }
}

