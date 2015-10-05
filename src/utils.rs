use std::result::Result;
use std::error::Error;

/// Extracts current result's value or maps the error
/// if the result contains an error and unwraps it.
///
/// # Examples
///
macro_rules! get {
    (
        $(
            $res: expr, $msg: expr
         );*
    ) => {
        $( $res.map_err(|e| format!("{}: {}", $msg, e)).unwrap() )*
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn should_extract() {
        let res: Result<i32, i32> = Ok(10);
        assert!(get!(res, "Oops!") == 10);
    }

    #[test]
    #[should_panic(expected = "Oops!")]
    fn should_panic() {
        let res: Result<i32, i32> = Err(10);
        get!(res, "Oops!");
    }
}
