use std::fmt::{ Display };

pub fn enrich_err<T: Display>(msg: &str) -> Box<Fn(T) -> String> {
    let owned = msg.to_owned();
    Box::new(move |e: T| format!("{}: {}", owned, e))
}

macro_rules! get {
    (
        $(
            $res: expr, $msg: expr
        );*
    ) => {
                $( $res.map_err(&*enrich_err($msg)).unwrap() )*
    }
}
