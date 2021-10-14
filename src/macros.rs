//! macros collection
//! 

macro_rules! diff {
    ($l:expr, $r:expr) => {
        if $l > $r {
            $l - $r
        } else {
            $r - $l
        }
    };
}