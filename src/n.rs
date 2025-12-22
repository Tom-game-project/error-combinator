use std::marker::PhantomData;

trait Check2<T, Pre> 
    where Self:Sized
{
    type State;
    type Error;

    fn check(self, value: CheckState2<T, Pre>)
        -> CheckOutcome<T, Self::State, Self::Error>;

    fn and<B, C>(self, b: B) -> And<Self, B, C>
    where
        B: Check2<T, Self::State>,
        C: CombineErrorBuilder<Self::Error, B::Error>
    {
        And { a: self, b, _combine:PhantomData }
    }

    fn or<B, C>(self, b: B) -> Or<Self, B, C>
        where 
            B: Check2<T, Self::State>,
            C: CombineErrorBuilder<Self::Error, B::Error>
    {
        Or { a: self, b, _combine: PhantomData }
    }
}

enum CheckOutcome<T, State, E> {
    Passed(CheckState2<T, State>),
    Failed{
        state: CheckState2<T, State>,
        err: E
    },
}

trait CombineError<EA, EB> {
    type Out;

    fn left(&mut self, ea: EA);
    fn right(&mut self, eb: EB);
    fn finish(self) -> Self::Out;
}

trait CombineErrorBuilder<EA, EB> {
    type Combiner: CombineError<EA, EB, Out = Self::Out>;
    type Out;

    fn build() -> Self::Combiner;
}

impl<E> CombineErrorBuilder<E, E> for DefaultCombine<E> {
    type Combiner = DefaultCombine<E>;
    type Out = E;

    fn build() -> Self::Combiner {
        DefaultCombine { data:None }
    }
}

struct DefaultCombine<E> {
    data: Option<E>
}

impl<E> CombineError<E, E> for DefaultCombine<E> {
    type Out = E;

    fn left(&mut self, ea: E) {
        self.data = Some(ea);
    }

    fn right(&mut self, eb: E) {
        self.data = Some(eb);
    }

    fn finish(self) -> Self::Out {
        self.data.unwrap()
    }
}

pub struct CheckState2<T: Sized, S> 
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

impl<T, Pre, A, B, C> Check2<T, Pre> for And<A, B, C>
where
    A: Check2<T, Pre>,
    B: Check2<T, A::State>,
    C: CombineErrorBuilder<A::Error, B::Error>,
{
    type State = B::State;
    type Error =  <C::Combiner as CombineError<A::Error, B::Error>>::Out;

    fn check(self, value: CheckState2<T, Pre>)
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
                            state: CheckState2 { value: state.value, _state: PhantomData },
                            err: combine.finish()
                        }
                    }
                }
            }
            CheckOutcome::Failed{state, err} => {
                // failed B
                combine.left(err);
                CheckOutcome::Failed{
                    state: CheckState2 { value: state.value, _state: PhantomData },
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

impl<T, Pre, A, B, C> Check2<T, Pre> for Or<A, B, C>
where
    A: Check2<T, Pre>,
    B: Check2<T, A::State>,
    C: CombineErrorBuilder<A::Error, B::Error>,
{
    type State = B::State;
    type Error = C::Out;

    fn check(self, value: CheckState2<T, Pre>)
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
                            state: CheckState2 { value: state.value, _state: PhantomData },
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
                            state: CheckState2 { value: state.value, _state: PhantomData },
                            err: combine.finish()
                        }
                    }
                }
            }
        }
    }
}

impl<T, Pre, State, F, E> Check2<T, Pre> for F
where
    F: Fn(CheckState2<T, Pre>) -> CheckOutcome<T, State, E>,
{
    type State = State;
    type Error = E;

    fn check(self, value: CheckState2<T, Pre>)
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
enum ValidateErr2 {
    CheckStartsWithHelloErr,
    CheckMin6Err,
}

fn check_starts_with_hello(
    data: CheckState2<&str, ErrState<unchecked, unchecked>>) 
-> 
CheckOutcome<&str, ErrState<checked, unchecked>, ValidateErr2>
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
CheckOutcome<&str, ErrState<checked, checked>, ValidateErr2>
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

struct CustomCombine<T> {
    data: Vec<T>
}

impl<E> CombineErrorBuilder<E, E> for CustomCombine<E> {
    type Combiner = CustomCombine<E>;
    type Out = Vec<E>;

    fn build() -> Self::Combiner {
        CustomCombine { data: Vec::new() }
    }
}

impl<T> CombineError<T, T> for CustomCombine<T> {
    type Out = Vec<T>;

    fn left(&mut self, ea: T) {
        self.data.push(ea);
    }

    fn right(&mut self, eb: T) {
        self.data.push(eb);
    }

    fn finish(self) -> Self::Out {
        self.data
    }
}

#[cfg(test)]
mod tests_n {
    use super::*;

    #[test]
    fn n_works00() {
        let s = "hello";

        let checker =
            check_starts_with_hello
            .or::<_, CustomCombine<ValidateErr2>>(check_min6);

            // .and::<_, DefaultCombine>(check_min6);

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

