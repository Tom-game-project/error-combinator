use std::marker::PhantomData;

use crate::ValidateErr;

trait Check2<T, Pre> 
    where Self:Sized
{
    type Pass;
    type Fail;
    type Error;

    fn check(self, value: CheckState2<T, Pre>)
        -> CheckOutcome<T, Self::Pass, Self::Fail, Self::Error>;

    fn and<B>(self, b: B) -> And<Self, B>
    where
        B: Check2<T, Self::Pass> {
        And { a: self, b }
    }
}

enum CheckOutcome<T, Pass, Fail, E> {
    Passed(CheckState2<T, Pass>),
    Failed{
        state: CheckState2<T, Fail>,
        err: E
    },
}

pub struct CheckState2<T: Sized, S> 
    where Self: Sized 
{
    value: T,
    _state: PhantomData<S>
}

enum AndFail<A, B> {
    Left(A),
    Right(B),
}

struct And<A, B> {
    a: A,
    b: B,
}

impl<T, Pre, A, B, E> Check2<T, Pre> for And<A, B>
where
    A: Check2<T, Pre, Error = E>,
    B: Check2<T, A::Pass, Error = E>,
{
    type Pass = B::Pass;
    type Fail = AndFail<A::Fail, B::Fail>; // TODO
    type Error = E;

    fn check(self, value: CheckState2<T, Pre>)
        -> CheckOutcome<T, Self::Pass, Self::Fail, Self::Error>
    {
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
                        CheckOutcome::Failed{
                            state: CheckState2 { value: state.value, _state: PhantomData },
                            err
                        }
                    }
                }
            }
            CheckOutcome::Failed{state, err} => {
                // failed B
                CheckOutcome::Failed{
                    state: CheckState2 { value: state.value, _state: PhantomData },
                    err
                }
            }
        }
    }
}

impl<T, Pre, Pass, Fail, F, E> Check2<T, Pre> for F
where
    F: Fn(CheckState2<T, Pre>) -> CheckOutcome<T, Pass, Fail, E>,
{
    type Pass = Pass;
    type Fail = Fail;
    type Error = E;

    fn check(self, value: CheckState2<T, Pre>)
        -> CheckOutcome<T, Self::Pass, Self::Fail, Self::Error>
    {
        self(value)
    }
}


struct checked;
struct unchecked;
struct ErrState<CheckStartsWithHello, CheckMin3> {
    _check_starts_with_hello: PhantomData<CheckStartsWithHello>,
    _check_min3: PhantomData<CheckMin3>
}

#[derive(Debug)]
enum ValidateErr2 {
    CheckStartsWithHelloErr,
    CheckMin6Err,
}

fn check_starts_with_hello(
    data: CheckState2<&str, ErrState<unchecked, unchecked>>) 
-> 
CheckOutcome<&str, ErrState<checked, unchecked>, ValidateErr, ValidateErr2>
{
    if data.value.starts_with("hello") {
        CheckOutcome::Passed(
            CheckState2 { value: data.value, _state: PhantomData }
        )
    } else {
        CheckOutcome::Failed{
            state: CheckState2 { value: data.value, _state: PhantomData },
            err: ValidateErr2::CheckStartsWithHelloErr
        }
    }
}

fn check_min6(
    data: CheckState2<&str, ErrState<checked, unchecked>>) 
-> 
CheckOutcome<&str, ErrState<checked, checked>, ValidateErr, ValidateErr2>
{
    if 6 < data.value.len() {
        CheckOutcome::Passed(
            CheckState2 { value: data.value, _state: PhantomData }
        )
    }
    else {
        CheckOutcome::Failed{
            state: CheckState2 { value: data.value, _state: PhantomData },
            err: ValidateErr2::CheckMin6Err
        }
    }
}

#[cfg(test)]
mod tests_n {
    use super::*;

    #[test]
    fn n_works00() {

        let s = "hello w";

        let checker = check_starts_with_hello
            .and(check_min6);
        let r = checker.check(
            CheckState2 { value: s, _state: PhantomData }
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


