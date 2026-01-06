use error_combinator::{
    check::{
        Check,
        CheckState,
        check_ref
    },
    cmberr::{
        CombineErrorBuilder,
        CombineError
    }
};

struct CheckStatus;

struct CheckStartsWithHelloErr (&'static str);

struct CheckEndsWithWorldErr (&'static str);

fn check_starts_with_hello(
    data: &str
)
-> Result<(), CheckStartsWithHelloErr>
{
    if data.starts_with("hello") {
        Ok(())
    } else {
        Err(CheckStartsWithHelloErr(
            "This does not starts with \"hello\""
        ))
    }
}

fn check_ends_with_world(
    data: &str
)
-> 
Result<(), CheckEndsWithWorldErr>
{
    if data.ends_with("world") {
        Ok(())
    } else {
        Err(CheckEndsWithWorldErr("This does not ends with \"world\""))
    }
}

pub struct CustomCombine<'a> {
    msgs: Vec<&'a str>
}

impl<'a> CombineErrorBuilder<CheckStartsWithHelloErr, CheckEndsWithWorldErr> for CustomCombine<'a> {
    type Combiner = Self;
    type Out = Vec<&'a str>;

    fn build() -> Self::Combiner {
        CustomCombine { msgs: Vec::new() }
    }
}

impl<'a> CombineError<CheckStartsWithHelloErr, CheckEndsWithWorldErr> for CustomCombine<'a> {
    type Out = Vec<&'a str>;


    fn left(&mut self, ea: CheckStartsWithHelloErr) {
        self.msgs.push(ea.0);
    }

    fn right(&mut self, eb: CheckEndsWithWorldErr) {
        self.msgs.push(eb.0);
    }

    fn finish(self) -> Self::Out {
        self.msgs
    }
}

// different types of errors
fn main() {
    let check_list  = [
        "hello abc world",
        "abc world",
        "hello world",
        "hello abc",
        "hello--",
        "abc----", 
        "world--"
    ];

    for s in check_list {
        let checker =
            check_ref::<
                str, 
                CheckStatus,
                CheckStatus,
                _, 
                _
            >(check_starts_with_hello)
            .or::<
                _, 
                CustomCombine
            >(
            check_ref::<
                str,
                CheckStatus,
                CheckStatus,
                _,
                _
            >(check_ends_with_world)
            );

        let r = checker.check(
            CheckState::new(s)
        );
        println!("test case: \"{}\"", s);
        match r.to_result() {
            Ok(v) => {
                println!("\"{}\" Passed!", v);
            }
            Err(err) => {
                println!("Failed because");
                println!("{:?}", err)
            }
        }
        println!("---")
    }
}
