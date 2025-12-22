use std::marker::PhantomData;

use crate::cmberr::{
    CombineErrorBuilder,
    CombineError
};

trait Check<T, Pre> 
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

enum CheckOutcome<T, State, E> {
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

struct And<A, B, C> {
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
                let tmp = self.b.check(v);

                match tmp {
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
                // failed B
                combine.left(err);
                CheckOutcome::Failed{
                    state: CheckState { value: state.value, _state: PhantomData },
                    err: combine.finish()
                }
            }
        }
    }
}

struct Or<A, B, C> {
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
                let tmp = self.b.check(v);

                match tmp {
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
                // failed B
                let tmp = self.b.check(state);
                combine.left(err);
                match tmp {
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

struct checked;
struct unchecked;
struct ErrState<CheckStartsWithHello, CheckMin3> {
    _check_starts_with_hello: PhantomData<CheckStartsWithHello>,
    _check_min3: PhantomData<CheckMin3>
}

#[derive(Debug)]
enum ValidateErr {
    CheckStartsWithHelloErr,
    CheckMin6Err,
}

fn check_starts_with_hello(
    data: CheckState<&str, ErrState<unchecked, unchecked>>) 
-> 
CheckOutcome<&str, ErrState<checked, unchecked>, ValidateErr>
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
    data: CheckState<&str, ErrState<checked, unchecked>>) 
-> 
CheckOutcome<&str, ErrState<checked, checked>, ValidateErr>
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

#[cfg(test)]
mod tests_n {
    use super::*;
    use crate::cmberr::CustomCombine;

    #[test]
    fn n_works00() {
        let s = "hello";

        let checker =
            check_starts_with_hello
            .or::<_, CustomCombine<ValidateErr>>(check_min6);

            // .and::<_, DefaultCombine>(check_min6);

        let r = checker.check(
            CheckState { value: s, _state: PhantomData }
        );

        match r {
            CheckOutcome::Passed(v) => {
                println!("Passed!");
            }
            CheckOutcome::Failed{state, err} => {
                println!("Failed because");
                println!("{:?}", err)
            }
        }
    }
}

