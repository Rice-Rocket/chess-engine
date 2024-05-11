/// Executes the given function on all squares, returning the sum.
/// 
/// Syntax: {`function`}: {`args`}
///
/// Note that `args` does not include the square.
#[macro_export]
macro_rules! sum_sqrs {
    ($eval:ident, $f:ident: $($arg:expr),* $(,)*) => {
        {
            let mut sum = 0i32;
            
            for sqr in Coord::iter_squares() {
                sum += $eval.$f($($arg,)* sqr);
            }

            sum
        }
    }
}

/// assert_eval!(`f`, [[`file`, `rank`]], `white_eval`, `black_eval`, `fen`; {`args`})
/// 
///   --> Test evaluation function `f` at the given `rank` and `file`.
///
/// assert_eval!(`f`, `white_eval`, `black_eval`, `fen`; {`args`})
///
///   --> Test evaluation function `f` over all squares, summing their results.
///
/// assert_eval!(- `f`, `white_eval`, `black_eval`, `fen`; {`args`})
///
///   --> Test evaluation function `f` without supplying any square arguments.
#[macro_export]
macro_rules! assert_eval {
    ($f:ident, [$file:expr, $rank:expr], $w:expr, $b:expr, $eval:ident $(; $($arg:expr),*)?) => {
        $eval.color = Color::White;
        $eval.init();
        assert_eq!($eval.$f(
            $($($arg,)*)? 
            Coord::new($file, $rank)
        ), $w);
        
        // $state.color = Color::Black;
        // assert_eq!($f(
        //     &$state,
        //     $($($arg,)*)? 
        //     Coord::new($file, $rank)
        // ), $b);
    };

    ($f:ident, $w:expr, $b:expr, $eval:ident $(; $($arg:expr),*)?) => {
        $eval.color = Color::White;
        $eval.init();
        assert_eq!(sum_sqrs!(
            $eval, $f:
            $($($arg,)*)? 
        ), $w);

        // $state.color = Color::Black;
        // assert_eq!(sum_sqrs!(
        //     $f:
        //     &$state,
        //     $($($arg,)*)? 
        // ), $b);
    };

    (- $f:ident, $w:expr, $b:expr, $eval:ident $(; $($arg:expr),*)?) => {
        $eval.color = Color::White;
        $eval.init();
        assert_eq!($eval.$f(
            $($($arg,)*)? 
        ), $w);

        // $state.color = Color::Black;
        // assert_eq!($f(
        //     &$state,
        //     $($($arg,)*)? 
        // ), $b);
    };
}


#[macro_export]
macro_rules! dbg_sqr_vals {
    ($f:ident, $state:ident $(; $($arg:expr),*)?) => {
        {
            let mut s = String::from("\n\r");
            for rank in (0..8).rev() {
                let mut row = String::from("");
                for file in 0..8 {
                    let sqr = Coord::new(file, rank);
                    let v = $f(&$state $($(,$arg)*)? , sqr);
                    if v == 0 {
                        row += "• ";
                    } else {
                        row += &format!("{} ", v);
                    }
                }
                s += &row;
                s += "\n\r";
            };
            println!("{}", s);
        }
    }
}
