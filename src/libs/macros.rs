// #[macro_use]
#[macro_export]
macro_rules! comp {
    [value:expr; for x in expr] => {
        {
            let mut res = Vec::new();
            for x in expr {
                res.push(value);
            }
            res
        }
    };
    [value:expr; for x in expr; if cond] => {
        {
            let mut res = Vec::new();
            for x in expr {
                if !cond { continue; }
                res.push(value);
            }
            res
        }
    };
    [$value:expr; while $cond:expr] => {
        {
            let mut res = Vec::new();
            while $cond {
                res.push($value);
            }
            res
        }
    };
    [$value:expr; until $cond:expr] => {
        {
            let mut res = Vec::new();
            while !$cond {
                res.push($value);
            }
            res
        }
    };
}

#[macro_use]
#[macro_export]
macro_rules! until {
    ($cond:expr; $action:block) => {
        while !$cond {
            $action
        }
    };
}

#[macro_use]
#[macro_export]
macro_rules! dbg {
    ($val:expr) => {{
        let value = &$val;
        eprintln!(
            "[{}:{}] {} = {:?}",
            file!(),
            line!(),
            stringify!($val),
            value
        );
        value
    }};
}

macro_rules! map {
    {$key:ident, $value:expr; for $x:ident in $to_iterate_over:expr} => {
        {
            let mut res = std::collections::HashMap::new();
            for $x in $to_iterate_over {
                res.insert($key, $value);
            }
            res
        }

    };
}

macro_rules! time_it {
    ($label:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let result = $code;
        println!("{} took: {:?}", $label, start.elapsed());
        result
    }};
}

macro_rules! pipe {
    ($value:expr => $func:ident) => {
        $func($value)
    };
    ($value:expr => $func:ident => $($rest:tt)*) => {
        pipe!($func($value) => $($rest)*)
    };
}
