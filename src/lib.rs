pub mod calculator_functions {
    pub mod basic_functions {
        pub fn add() {}
        pub fn subtract() {}
        pub fn divide() {}
        pub fn multiply() {}
    }

    pub mod power_functions {
        pub fn square_function() {}
        pub fn cube_function() {}
        pub fn power_function() {}
    }
}

use crate::calculator_functions::basic_functions;
use crate::calculator_functions::power_functions as p_func;

pub fn make_some_basic_calculation() {
    basic_functions::add();
    basic_functions::subtract();
    basic_functions::divide();
    basic_functions::multiply();
}

pub fn make_some_power_calculation() {
    p_func::square_function();
    p_func::cube_function();
    p_func::power_function();
}