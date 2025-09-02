pub mod variables;
pub mod basic_types;
pub mod controls;
pub mod methods;
pub mod compound_types;


pub fn run_all() {
    println!("运行 basics 模块所有示例:");
    variables::ex1_variable_run();
    variables::ex2_name_rules();
    variables::ex3_unpack_run();
    variables::ex4_const_run();
    variables::ex5_variable_shadowing_run();
    variables::ownership_borrow();

    basic_types::char_type();
    basic_types::functions_type();
    basic_types::unit_type();
    basic_types::statements_expressions_type();

    compound_types::array_type();
    compound_types::enum_type();
    compound_types::string_slice_type();
    compound_types::struct_type();
    compound_types::tuple_type();


    controls::condition_control_run();
    controls::loop_control_run();
}