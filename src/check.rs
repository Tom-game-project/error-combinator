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

pub struct ResultCheck<F, T, PreState, PostState, E>
{
    func: F,
    _pre: PhantomData<PreState>,
    _state: PhantomData<PostState>,
    _func_arg: PhantomData<T>,
    _func_ret: PhantomData<E>,
}

pub struct ResultCheckBuilder<PreState, PostState> {
    _pre_state: PhantomData<PreState>,
    _post_state: PhantomData<PostState>,
}


impl<PreState, PostState> ResultCheckBuilder<PreState, PostState> {
    pub fn build<F, T, E>(func: F) -> ResultCheck<F, T, PreState, PostState, E>
        where F: Fn(&T) -> Result<(), E>
    {
        let s = Self { _pre_state: PhantomData, _post_state: PhantomData };
        ResultCheck { 
            func: func, 
            _pre: s._pre_state, 
            _state: s._post_state, 
            _func_arg: PhantomData, 
            _func_ret: PhantomData 
        }
    }
}

impl<T, PreState, PostState, F, E> Check<T, PreState> for ResultCheck<F, T, PreState, PostState, E>
where
    F: Fn(&T) -> Result<(), E>,
{
    type PostState = PostState;
    type Error = E;

    fn check(self, value: CheckState<T, PreState>)
            -> CheckOutcome<T, Self::PostState, Self::Error> {
          match (self.func)(&value.value) {
             Ok(_v) => {
                 CheckOutcome::Passed(
                     CheckState::new(value.value)
                 )
             },
             Err(e) => CheckOutcome::Failed {
                 state: CheckState::new(value.value),
                 err: e,
             },
         }
    }
}

