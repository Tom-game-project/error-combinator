use std::marker::PhantomData;

pub mod n;

pub trait Check<T, PreCheckState>
    where Self: Sized
{
    type Next;
    type Error;

    fn check(&self, value: CheckState<T, PreCheckState>) -> Result<CheckState<T, Self::Next>, Self::Error>;

    fn and<B>(self, b: B) -> And<Self, B, Self::Error>
    where
        B: Check<T, Self::Next, Error = Self::Error>,
    {
        And { a: self, b, _err: PhantomData }
    }
}

pub struct CheckState<T: Sized, S> 
    where Self: Sized 
{
    value: T,
    _state: PhantomData<S> // checked or not
}

pub struct And<A, B, E> {
    a: A,
    b: B,
    _err: PhantomData<E>
}

impl<T, E, Pre, Next, F> Check<T, Pre> for F
    where F: Fn(CheckState<T, Pre>) -> Result<CheckState<T, Next>, E>,
{
    type Next = Next;
    type Error = E;

    fn check(&self, value: CheckState<T, Pre>) -> Result<CheckState<T, Self::Next>, Self::Error> {
        self(value)
    }
}

impl <T, A, B, E, Pre, Mid, Next> Check<T, Pre> for And<A, B, E>
    where A: Check<T, Pre, Error = E, Next = Mid>,
          B: Check<T, Mid, Error = E, Next = Next>,
{
    type Next = Next;
    type Error = E;

    fn check(&self, value: CheckState<T, Pre>) -> Result<CheckState<T, Self::Next>, Self::Error> {
        match self.a.check(value) {
            Ok(a) => {
                match self.b.check(a) {
                    Ok(b) => {
                        return Ok(b);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

// ------------------------------------

enum ValidateErr {
    NotStartsWithHello,
    NotEnoughLength,
}

struct checked;
struct unchecked;

pub struct ErrState<CheckStartsWithHello, CheckMin3> {
    _check_starts_with_hello: PhantomData<CheckStartsWithHello>,
    _check_min3: PhantomData<CheckMin3>
}

fn check_starts_with_hello(
    data: CheckState<&str, ErrState<unchecked, unchecked>>) 
-> Result<CheckState<&str, ErrState<checked, unchecked>>, ValidateErr> 
{
    if data.value.starts_with("hello") {
        Ok(
            CheckState { value: data.value,
            _state: PhantomData 
        }
        )
    } else {
        Err(ValidateErr::NotStartsWithHello)
    }
}

fn check_min3(
    data: CheckState<&str, ErrState<checked, unchecked>>
    )
-> Result<CheckState<&str, ErrState<checked, checked>>, ValidateErr> 
{
    if 0 < data.value.len() {
        Ok(
            CheckState {
                value: data.value,
                _state: PhantomData 
            }
        )
    }
    else {
        Err(ValidateErr::NotEnoughLength)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works00() {
        {
            let data = "hello world";
            let data: CheckState<&str, ErrState<unchecked, unchecked>> = CheckState{value:data, _state: PhantomData};
            let a = check_starts_with_hello;

            let result = a.check(data);
            assert!(result.is_ok());
        }
        {
            let data = "world hello";
            let data: CheckState<&str, ErrState<unchecked, unchecked>> = CheckState{value:data, _state: PhantomData};
            let a = check_starts_with_hello;

            let result = a.check(data);
            assert!(result.is_err());
        }
        // println!("Ok!!!");
    }

    #[test]
    fn it_works01() {
        let data = "hello world";
        let data: CheckState<&str, ErrState<unchecked, unchecked>> = CheckState{value:data, _state: PhantomData};
        let a = 
            check_starts_with_hello
            .and(check_min3);
        let result = a.check(data);

        assert!(result.is_ok());
    }
}

