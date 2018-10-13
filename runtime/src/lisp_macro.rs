#[macro_export]
macro_rules! lisp {
    // bool
    ($scope:expr, false) => {
        $scope.get("false").unwrap().clone()
    };
    ($scope:expr, true) => {
        $scope.get("true").unwrap().clone()
    };

    ($scope:expr, str $value:expr) => {{
        let string = $value.to_owned();
        let string_kind = unsafe {
            $scope
                .get_with_type::<$crate::Kind>("String")
                .unwrap()
        };
        unsafe { $crate::gc::Gc::new($crate::Object::new(string_kind, string)) }.into_value()
    }};

    ($scope:expr, symb $value:expr) => {{
        let symbol = $value.to_owned();
        let symbol_kind = unsafe {
            $scope
                .get_with_type::<$crate::Kind>("Symbol")
                .unwrap()
        };
        unsafe { $crate::gc::Gc::new($crate::Object::new(symbol_kind, symbol)) }.into_value()
    }};

    ($scope:expr, $func:ident) => {{
        let symbol = stringify!($func).to_owned();
        let symbol_kind = unsafe {
            $scope
                .get_with_type::<$crate::Kind>("Symbol")
                .unwrap()
        };
        unsafe { $crate::gc::Gc::new($crate::Object::new(symbol_kind, symbol)) }.into_value()
    }};

    ($scope:expr, ( $($e:tt)* ) ) => {{
        let list_kind = unsafe {
            $scope
                .get_with_type::<$crate::Kind>("List")
                .unwrap()
        };
        let mut list = unsafe { $crate::gc::Gc::new($crate::Object::new(list_kind, List::new())) };
        $( list.push_back( lisp!($scope, $e) ) );*;
        list.into_value()
    }};
}
