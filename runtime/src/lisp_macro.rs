#[macro_export]
macro_rules! lisp {
    ($scope:expr, true) => {
        $crate::context::new_true($scope)
    };

    ($scope:expr, false) => {
        $crate::context::new_false($scope)
    };

    ($scope:expr, $symbol:ident) => {
        $crate::context::new_symbol($scope, stringify!($symbol))
    };

    ($scope:expr, "$string:ident") => {
        $crate::context::new_string($scope, stringify!($string))
    };

    ($scope:expr, ( $( $t:tt )* )) => {{
        let mut list = $crate::context::new_list($scope);
        $( list.push_back(lisp!($scope, $t).into_value()); )*
        list
    }};
}
