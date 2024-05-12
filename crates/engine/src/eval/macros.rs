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
    };
    
    (+ $eval:ident, $f:ident: $($arg:expr),* $(,)*) => {
        {
            let mut sum = 0i32;
            
            for sqr in Coord::iter_squares() {
                sum += $eval.$f($($arg,)* sqr).count() as i32;
            }

            sum
        }
    };

    (* [$($count:tt),+ $(,)*] $eval:ident, $f:ident: $($arg:expr),* $(,)*) => {
        {
            let mut sums = ($($count,)*);

            $(
                let mut sum = 0i32;

                for sqr in Coord::iter_squares() {
                    sum += $eval.$f(sqr).$count.count() as i32;
                }

                sums.$count = sum;
            )*

            sums
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
        
        $eval.color = Color::Black;
        assert_eq!($eval.$f(
            $($($arg,)*)? 
            Coord::new($file, $rank)
        ), $b);
    };

    (+ $f:ident, [$file:expr, $rank:expr], $w:expr, $b:expr, $eval:ident $(; $($arg:expr),*)?) => {
        $eval.color = Color::White;
        $eval.init();
        assert_eq!(if $eval.$f(
            $($($arg,)*)? 
        ).contains_square(Coord::new($file, $rank).square()) { 1 } else { 0 }, $w);
        
        $eval.color = Color::Black;
        assert_eq!(if $eval.$f(
            $($($arg,)*)? 
        ).contains_square(Coord::new($file, $rank).square()) { 1 } else { 0 }, $b);
    };

    ($f:ident, $w:expr, $b:expr, $eval:ident $(; $($arg:expr),*)?) => {
        $eval.color = Color::White;
        $eval.init();
        assert_eq!(sum_sqrs!(
            $eval, $f:
            $($($arg,)*)? 
        ), $w);

        $eval.color = Color::Black;
        assert_eq!(sum_sqrs!(
            $eval, $f:
            $($($arg,)*)? 
        ), $b);
    };

    (+ $f:ident, $w:expr, $b:expr, $eval:ident $(; $($arg:expr),*)?) => {
        $eval.color = Color::White;
        $eval.init();
        assert_eq!(sum_sqrs!( +
            $eval, $f:
            $($($arg,)*)? 
        ), $w);

        $eval.color = Color::Black;
        assert_eq!(sum_sqrs!( +
            $eval, $f:
            $($($arg,)*)? 
        ), $b);
    };

    (* [$($count:tt),+] $f:ident, $w:expr, $b:expr, $eval:ident $(; $($arg:expr),*)?) => {
        $eval.color = Color::White;
        $eval.init();
        assert_eq!(sum_sqrs!( * [$($count,)+]
            $eval, $f:
            $($($arg,)*)?
        ), $w);

        $eval.color = Color::Black;
        assert_eq!(sum_sqrs!( * [$($count,)+]
            $eval, $f:
            $($($arg,)*)?
        ), $w);
    };

    (- $f:ident, $w:expr, $b:expr, $eval:ident $(; $($arg:expr),*)?) => {
        $eval.color = Color::White;
        $eval.init();
        assert_eq!($eval.$f(
            $($($arg,)*)? 
        ), $w);

        $eval.color = Color::Black;
        assert_eq!($eval.$f(
            $($($arg,)*)? 
        ), $b);
    };

    (+ - $f:ident, $w:expr, $b:expr, $eval:ident $(; $($arg:expr),*)?) => {
        $eval.color = Color::White;
        $eval.init();
        assert_eq!($eval.$f(
            $($($arg,)*)? 
        ).count() as i32, $w);

        $eval.color = Color::Black;
        assert_eq!($eval.$f(
            $($($arg,)*)? 
        ).count() as i32, $b);
    };

    (* - [$($count:tt),+] $f:ident, $w:expr, $b:expr, $eval:ident $(; $($arg:expr),*)?) => {
        $eval.color = Color::White;
        $eval.init();
        $(
            assert_eq!($eval.$f().$count.count() as i32, $w.$count);
        )*

        $eval.color = Color::Black;
        $(
            assert_eq!($eval.$f().$count.count() as i32, $b.$count);
        )*
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
