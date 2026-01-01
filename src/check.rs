use std::marker::PhantomData;

use crate::cmberr::{
    CombineErrorBuilder,
    CombineError
};

pub trait Check<T, PreState> 
    where Self:Sized
{
    type PostState;
    type Error;

    fn check(self, value: CheckState<T, PreState>)
        -> CheckOutcome<T, Self::PostState, Self::Error>;

    fn and<B, C>(self, b: B) -> And<Self, B, C>
        where
            B: Check<T, Self::PostState>,
            C: CombineErrorBuilder<Self::Error, B::Error>
    {
        And { a: self, b, _combine:PhantomData }
    }

    fn or<B, C>(self, b: B) -> Or<Self, B, C>
        where 
            B: Check<T, Self::PostState>,
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

impl<T, State, E> CheckOutcome<T, State, E> {
    pub fn to_result(self) -> Result<T, E> {
        match self {
            CheckOutcome::Passed(v) => {
                Ok(v.value)
            }
            CheckOutcome::Failed{state:_, err} => {
                Err(err)
            }
        }
    }

    pub fn to_result_with_data(self) -> Result<T, (T, E)> {
        match self {
            CheckOutcome::Passed(v) => {
                Ok(v.value)
            }
            CheckOutcome::Failed{state, err} => {
                Err((state.value, err))
            }
        }
    }
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

impl<T, PreState, A, B, C> Check<T, PreState> for And<A, B, C>
where
    A: Check<T, PreState>,
    B: Check<T, A::PostState>,
    C: CombineErrorBuilder<A::Error, B::Error>,
{
    type PostState = B::PostState;
    type Error =  <C::Combiner as CombineError<A::Error, B::Error>>::Out;

    fn check(self, value: CheckState<T, PreState>)
        -> CheckOutcome<T, Self::PostState, Self::Error>
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

impl<T, PreState, A, B, C> Check<T, PreState> for Or<A, B, C>
where
    A: Check<T, PreState>,
    B: Check<T, A::PostState>,
    C: CombineErrorBuilder<A::Error, B::Error>,
{
    type PostState = B::PostState;
    type Error = C::Out;

    fn check(self, value: CheckState<T, PreState>)
        -> CheckOutcome<T, Self::PostState, Self::Error>
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

impl<T, PreState, PostState, F, E> Check<T, PreState> for F
where
    F: Fn(CheckState<T, PreState>) -> CheckOutcome<T, PostState, E>,
{
    type PostState = PostState;
    type Error = E;

    fn check(self, value: CheckState<T, PreState>)
        -> CheckOutcome<T, Self::PostState, Self::Error>
    {
        self(value)
    }
}

pub fn check_ref<'a, T: ?Sized, Pre, Post, E, F>(
    f: F
) -> impl Check<&'a T, Pre, PostState = Post, Error = E>
where
    F: Fn(&T) -> Result<(), E>,
{
    move |state: CheckState<&'a T, Pre>| {
        match f(state.value) {
            Ok(()) => CheckOutcome::Passed(CheckState::new(state.value)),
            Err(e) => CheckOutcome::Failed {
                state: CheckState::new(state.value),
                err: e,
            },
        }
    }
}


pub fn check_noref<T: Sized, Pre, Post, E, F>( 
    f: F
) -> impl Check<T, Pre, PostState = Post, Error = E>
where 
    F: Fn(&T) -> Result<(), E>,
{
    move |state: CheckState<T, Pre>| {
          match f(&state.value) {
             Ok(_v) => {
                 CheckOutcome::Passed(
                     CheckState::new(state.value)
                 )
             },
             Err(e) => CheckOutcome::Failed {
                 state: CheckState::new(state.value),
                 err: e,
             },
         }
    }
}
