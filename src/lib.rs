pub trait Check<T:?Sized> {
    type Error;

    fn check(&self, value: &T) -> Result<(), Self::Error>;
}

impl<T:?Sized, E, F> Check<T> for F
    where F: Fn(&T) -> Result<(), E>,
{
    type Error = E;

    fn check(&self, value: &T) -> Result<(), E> {
        self(value)
    }
}

pub enum ValidateErr {
    NotStartsWithHello,
}

pub fn check_starts_with_hello(data: &str) -> Result<(), ValidateErr> {
    if data.starts_with("hello") {
        Ok(())
    } else {
        Err(ValidateErr::NotStartsWithHello)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
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
}

