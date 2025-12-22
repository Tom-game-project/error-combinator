use std::marker::PhantomData;

use crate::ValidateErr;

trait Check2<T, Pre> 
    where Self:Sized
{
    type Pass;
    type Fail;

    fn check(self, value: CheckState2<T, Pre>)
        -> CheckOutcome<T, Self::Pass, Self::Fail>;

    fn and<B>(self, b: B) -> And<Self, B>
    where
        B: Check2<T, Self::Pass> {
        And { a: self, b }
    }
}

enum CheckOutcome<T, Pass, Fail> {
    Passed(CheckState2<T, Pass>),
    Failed(CheckState2<T, Fail>),
}

pub struct CheckState2<T: Sized, S> 
    where Self: Sized 
{
    value: T,
    _state: PhantomData<S>
}

struct AndFail<A, B> {
    _left: PhantomData<A>,
    _right: PhantomData<B>,
}

struct And<A, B> {
    a: A,
    b: B,
}

impl<T, Pre, A, B> Check2<T, Pre> for And<A, B>
where
    A: Check2<T, Pre>,
    B: Check2<T, A::Pass>,
{
    type Pass = B::Pass;
    type Fail = AndFail<A::Fail, B::Fail>; // TODO

    fn check(self, value: CheckState2<T, Pre>)
        -> CheckOutcome<T, Self::Pass, Self::Fail>
    {
        match self.a.check(value) {
            CheckOutcome::Passed(v) => {
                let tmp = self.b.check(v);

                match tmp {
                    CheckOutcome::Passed(vv) => {
                        // success A and success B
                        CheckOutcome::Passed(vv)
                    }
                    CheckOutcome::Failed(ff) => {
                        // success A and failed B
                        CheckOutcome::Failed(
                            CheckState2 { value: ff.value, _state: PhantomData }
                        )
                    }
                }
            }
            CheckOutcome::Failed(f) => {
                // failed B
                CheckOutcome::Failed(
                    CheckState2 { value: f.value, _state: PhantomData }
                )
            }
        }
    }
}

impl<T, Pre, Pass, Fail, F> Check2<T, Pre> for F
where
    F: Fn(CheckState2<T, Pre>) -> CheckOutcome<T, Pass, Fail>,
{
    type Pass = Pass;
    type Fail = Fail;

    fn check(self, value: CheckState2<T, Pre>)
        -> CheckOutcome<T, Self::Pass, Self::Fail>
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

fn check_starts_with_hello(
    data: CheckState2<&str, ErrState<unchecked, unchecked>>) 
-> 
CheckOutcome<&str, ErrState<checked, unchecked>, ValidateErr>
{
    if data.value.starts_with("hello") {
        CheckOutcome::Passed(
            CheckState2 { value: data.value, _state: PhantomData }
        )
    } else {
        CheckOutcome::Failed(
            CheckState2 { value: data.value, _state: PhantomData }
        )
    }
}

fn check_min6(
    data: CheckState2<&str, ErrState<checked, unchecked>>) 
-> 
CheckOutcome<&str, ErrState<checked, checked>, ValidateErr>
{
    if 6 < data.value.len() {
        CheckOutcome::Passed(
            CheckState2 { value: data.value, _state: PhantomData }
        )
    }
    else {
        CheckOutcome::Failed(
            CheckState2 { value: data.value, _state: PhantomData }
        )
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
            CheckOutcome::Failed(f) => {
                println!("Failed");
            }
        }
    }
}


