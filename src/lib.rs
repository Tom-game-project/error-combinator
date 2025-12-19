pub trait Check<T:?Sized> 
    where Self: Sized
{
    type Error;

    fn check(&self, value: &T) -> Result<(), Self::Error>;
    fn and<A>(self, other: A) -> And<Self, A>
        where A:Check<T, Error = Self::Error> {
            And { a: self, b: other }
    }
}

impl<T:?Sized, E, F> Check<T> for F
    where F: Fn(&T) -> Result<(), E>,
{
    type Error = E;

    fn check(&self, value: &T) -> Result<(), E> {
        self(value)
    }
}

// use std::marker::PhantomData;

struct And<A, B> {
    a: A,
    b: B,
}

impl <T:?Sized, A, B, E> Check<T> for And<A, B>
    where A: Check<T, Error = E>,
          B: Check<T, Error = E>,
{
    type Error = E;

    fn check(&self, value: &T) -> Result<(), Self::Error> {
        self.a.check(value)?;
        self.b.check(value)?;
        Ok(())
    }
}

// ------------------------------------

pub enum ValidateErr {
    NotStartsWithHello,
    NotEnoughtLength,
}

pub fn check_starts_with_hello(data: &str) -> Result<(), ValidateErr> {
    if data.starts_with("hello") {
        Ok(())
    } else {
        Err(ValidateErr::NotStartsWithHello)
    }
}

pub fn check_min3(data: &str) -> Result<(), ValidateErr> {
    if 0 < data.len() {
        Ok(())
    } else {
        Err(ValidateErr::NotEnoughtLength)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works00() {
        {
            let data = "hello world";
            let a = check_starts_with_hello;

            // ここが修正点
            let result = a.check(&data);
            assert!(result.is_ok());
        }
        {
            let data = "world hello";
            let a = check_starts_with_hello;

            // ここが修正点
            let result = a.check(&data);
            assert!(result.is_err());
        }
        // println!("Ok!!!");
    }

    #[test]
    fn it_works01() {
        let data = "hello world";
        let a = 
            check_starts_with_hello
            .and(check_min3);
        let result = a.check(&data);

        assert!(result.is_ok());
    }
}

