use prolog::ast::*;
use prolog::num::bigint::{BigInt};

use std::collections::HashMap;
use std::rc::Rc;

fn get_builtins() -> Code {
    vec![internal_call_n!(), // callN/N, 0.
         is_atomic!(temp_v!(1)), // atomic/1, 1.
         proceed!(),
         is_var!(temp_v!(1)),    // var/1, 3.
         proceed!(),
         allocate!(4), // catch/3, 5.
         fact![get_var_in_fact!(perm_v!(2), 1),
               get_var_in_fact!(perm_v!(3), 2),
               get_var_in_fact!(perm_v!(1), 3)],
         query![put_var!(perm_v!(4), 1)],
         get_current_block!(),
         query![put_value!(perm_v!(2), 1),
                put_value!(perm_v!(3), 2),
                put_value!(perm_v!(1), 3),
                put_unsafe_value!(4, 4)],
         deallocate!(),
         goto_execute!(12, 4), // goto catch/4.
         try_me_else!(10), // catch/4, 12.
         allocate!(3),
         fact![get_var_in_fact!(perm_v!(3), 1),
               get_var_in_fact!(perm_v!(2), 4)],
         query![put_var!(perm_v!(1), 1)],
         install_new_block!(),
         query![put_value!(perm_v!(3), 1)],
         call_n!(1),
         query![put_value!(perm_v!(2), 1),
                put_unsafe_value!(1, 2)],
         deallocate!(),
         goto_execute!(44, 2), //21: goto end_block/2.
         default_trust_me!(),
         allocate!(3),
         fact![get_var_in_fact!(perm_v!(2), 2),
               get_var_in_fact!(perm_v!(1), 3)],
         query![get_var_in_query!(temp_v!(2), 1),
                put_value!(temp_v!(4), 1)],
         reset_block!(),
         query![put_var!(perm_v!(3), 1)],
         get_ball!(),
         query![put_unsafe_value!(3, 1),
                put_value!(perm_v!(2), 2),
                put_value!(perm_v!(1), 3)],
         deallocate!(),
         goto_execute!(32, 2), // goto handle_ball/2.
         try_me_else!(10), // handle_ball/2, 32.
         allocate!(2),
         get_level!(perm_v!(1)),
         fact![get_var_in_fact!(perm_v!(2), 3)],
         unify!(),
         cut!(perm_v!(1)),
         erase_ball!(),
         query![put_value!(perm_v!(2), 1)],
         deallocate!(),
         execute_n!(1),
         default_trust_me!(),
         unwind_stack!(),
         try_me_else!(9), // end_block/2, 44.
         allocate!(1),
         fact![get_var_in_fact!(perm_v!(1), 1)],
         query![put_value!(temp_v!(2), 1)],
         clean_up_block!(),
         query![put_value!(perm_v!(1), 1)],
         deallocate!(),
         reset_block!(),
         proceed!(),
         default_trust_me!(), // 53.
         allocate!(0),
         query![get_var_in_query!(temp_v!(3), 1),
                put_value!(temp_v!(2), 1)],
         reset_block!(),
         deallocate!(),
         goto_execute!(61, 0),
         set_ball!(), // throw/1, 59.
         unwind_stack!(),
         fail!(), // false/0, 61.
         try_me_else!(7), // not/1, 62.
         allocate!(1),
         get_level!(perm_v!(1)),
         call_n!(1),
         cut!(perm_v!(1)),
         deallocate!(),
         goto_execute!(61, 0),
         trust_me!(),
         proceed!(),
         duplicate_term!(), // duplicate_term/2, 71.
         proceed!(),
         fact![get_value!(temp_v!(1), 2)], // =/2, 73.
         proceed!(),
         proceed!(), // true/0, 75.
         get_cp!(temp_v!(3)), // ','/2, 76.
         try_me_else!(18), // ','/3, 77.
         switch_on_term!(4, 1, 0, 0),
         indexed_try!(4),
         retry!(7),
         trust!(10),
         try_me_else!(4),
         fact![get_constant!(atom!("!"), temp_v!(1)),
               get_structure!(",", 2, temp_v!(2), Some(infix!())),
               unify_variable!(temp_v!(1)),
               unify_variable!(temp_v!(2))],
         set_cp!(temp_v!(3)),
         goto_execute!(77, 3),
         retry_me_else!(4),
         fact![get_constant!(atom!("!"), temp_v!(1)),
               get_constant!(atom!("!"), temp_v!(2))],
         set_cp!(temp_v!(3)),
         proceed!(),
         trust_me!(),
         fact![get_constant!(atom!("!"), temp_v!(1))],
         set_cp!(temp_v!(3)),
         query![put_value!(temp_v!(2), 1)],
         execute_n!(1),
         retry_me_else!(8), // 95.
         allocate!(3),
         fact![get_structure!(",", 2, temp_v!(2), Some(infix!())),
               unify_variable!(perm_v!(2)),
               unify_variable!(perm_v!(1)),
               get_var_in_fact!(perm_v!(3), 3)],
         neck_cut!(),
         call_n!(1),
         query![put_unsafe_value!(2, 1),
                put_unsafe_value!(1, 2),
                put_value!(perm_v!(3), 3)],
         deallocate!(),
         goto_execute!(77, 3),
         retry_me_else!(10),
         allocate!(2),
         get_level!(perm_v!(2)),
         fact![get_constant!(atom!("!"), temp_v!(2)),
               get_var_in_fact!(perm_v!(1), 3)],
         neck_cut!(),
         call_n!(1),
         query![put_value!(perm_v!(1), 1)],
         set_cp!(temp_v!(1)),
         deallocate!(),
         proceed!(),
         trust_me!(),
         allocate!(1),
         fact![get_var_in_fact!(perm_v!(1), 2)],
         call_n!(1),
         query![put_value!(perm_v!(1), 1)],
         deallocate!(),
         execute_n!(1),
         get_cp!(temp_v!(3)), // ';'/2, 120.
         try_me_else!(16), // ';'/3, 121.
         switch_on_term!(0, 12, 0, 1), // Fail on variable input.
         indexed_try!(2),
         trust!(5),
         fact![get_structure!("->", 2, temp_v!(1), Some(infix!())),
               unify_variable!(temp_v!(1)),
               unify_variable!(temp_v!(4))],
         query![put_value!(temp_v!(4), 2)],
         goto_execute!(147, 3), // goto '->'/3.
         retry_me_else!(5),
         fact![get_structure!("->", 2, temp_v!(1), Some(infix!())),
               unify_void!(2)],
         set_cp!(temp_v!(3)),
         query![put_value!(temp_v!(2), 1)],
         execute_n!(1),
         retry_me_else!(4),
         fact![get_constant!(atom!("!"), temp_v!(1))],
         set_cp!(temp_v!(3)),
         proceed!(),
         retry_me_else!(4),
         fact![get_constant!(atom!("!"), temp_v!(2))],
         set_cp!(temp_v!(3)),
         proceed!(),
         retry_me_else!(2),
         execute_n!(1),
         trust_me!(),
         query![put_value!(temp_v!(2), 1)],
         execute_n!(1),
         get_cp!(temp_v!(3)), // '->'/2, 146.
         try_me_else!(7), // '->'/3, 147.
         allocate!(1),
         fact![get_constant!(atom!("!"), temp_v!(2)),
               get_var_in_fact!(perm_v!(1), 3)],
         call_n!(1),
         set_cp!(perm_v!(1)),
         deallocate!(),
         proceed!(),
         trust_me!(),
         allocate!(2),
         fact![get_var_in_fact!(perm_v!(1), 2),
               get_var_in_fact!(perm_v!(2), 3)],
         call_n!(1),
         set_cp!(perm_v!(2)),
         query![put_unsafe_value!(1, 1)],
         deallocate!(),
         execute_n!(1),
         functor_execute!(), // functor/3, 162.
         is_integer!(temp_v!(1)), // integer/1, 163.
         proceed!(),
         get_arg_execute!(), // get_arg/3, 165.
         try_me_else!(10), // arg/3, 166.
         allocate!(4),
         fact![get_var_in_fact!(perm_v!(1), 1),
               get_var_in_fact!(perm_v!(2), 2),
               get_var_in_fact!(perm_v!(4), 3)],
         is_var!(perm_v!(1)),
         neck_cut!(),
         query![put_value!(perm_v!(2), 1),
                put_var!(temp_v!(4), 2),
                put_var!(perm_v!(3), 3)],
         functor_call!(),
         query![put_value!(perm_v!(1), 1),
                put_constant!(Level::Shallow, integer!(1), temp_v!(2)),
                put_unsafe_value!(3, 3),
                put_value!(perm_v!(2), 4),
                put_value!(perm_v!(4), 5)],
         deallocate!(),
         goto_execute!(189, 5), // goto arg_/5, 175.
         retry_me_else!(10),
         allocate!(3),
         fact![get_var_in_fact!(perm_v!(1), 1),
               get_var_in_fact!(perm_v!(2), 2),
               get_var_in_fact!(perm_v!(3), 3)],
         is_integer!(perm_v!(1)),
         neck_cut!(),
         query![put_value!(perm_v!(2), 1),
                put_var!(temp_v!(4), 2),
                put_var!(temp_v!(3), 3)],
         functor_call!(),
         query![put_value!(perm_v!(1), 1),
                put_value!(perm_v!(2), 2),
                put_value!(perm_v!(3), 3)],
         deallocate!(),
         goto_execute!(165, 3), // goto get_arg/3, 185.
         trust_me!(),
         query![get_var_in_query!(temp_v!(4), 1),
                put_structure!("type_error", 1, temp_v!(1), None),
                set_constant!(atom!("integer_expected"))],
         goto_execute!(59, 1), // goto throw/1.
         try_me_else!(5), // arg_/5, 189.
         fact![get_value!(temp_v!(1), 2),
               get_value!(temp_v!(1), 3)],
         neck_cut!(),
         query![put_value!(temp_v!(4), 2),
                put_value!(temp_v!(5), 3)],
         goto_execute!(165, 3), // goto get_arg/3.
         retry_me_else!(4),
         fact![get_value!(temp_v!(1), 2)],
         query![put_value!(temp_v!(4), 2),
                get_var_in_query!(temp_v!(6), 3),
                put_value!(temp_v!(5), 3)],
         goto_execute!(165, 3), // goto get_arg/3, 197.
         trust_me!(),
         allocate!(5),
         fact![get_var_in_fact!(perm_v!(2), 1),
               get_var_in_fact!(perm_v!(4), 3),
               get_var_in_fact!(perm_v!(3), 4),
               get_var_in_fact!(perm_v!(5), 5)],
         compare_number_instr!(CompareNumberQT::LessThan,
                               ArithmeticTerm::Reg(temp_v!(2)),
                               ArithmeticTerm::Reg(perm_v!(4))),
         add!(ArithmeticTerm::Reg(temp_v!(2)),
              ArithmeticTerm::Number(rc_integer!(1)),
              1),
         query![put_var!(perm_v!(1), 1)],
         is_call!(perm_v!(1), interm!(1)),
         query![put_value!(perm_v!(2), 1),
                put_unsafe_value!(1, 2),
                put_value!(perm_v!(4), 3),
                put_value!(perm_v!(3), 4),
                put_value!(perm_v!(5), 5)],
         deallocate!(),
         goto_execute!(189, 5), // goto arg_/5, 207.
         display!(), // display/1, 208.
         proceed!(),
         dynamic_is!(), // is/2, 210.
         proceed!(),
         dynamic_num_test!(cmp_gt!()), // >/2, 212.
         proceed!(),
         dynamic_num_test!(cmp_lt!()), // </2, 214.
         proceed!(),
         dynamic_num_test!(cmp_gte!()), // >=/2, 216.
         proceed!(),
         dynamic_num_test!(cmp_lte!()), // =</2, 218.
         proceed!(),
         dynamic_num_test!(cmp_ne!()), // =\=, 220.
         proceed!(),
         dynamic_num_test!(cmp_eq!()), // =:=, 222.
         proceed!(),
         try_me_else!(5), // =.., 224.
         fact![get_var_in_fact!(temp_v!(3), 1),
               get_list!(Level::Shallow, temp_v!(2)),
               unify_value!(temp_v!(3)),
               unify_constant!(Constant::EmptyList)],
         is_atomic!(temp_v!(3)),
         neck_cut!(),
         proceed!(),
         retry_me_else!(11),
         allocate!(4),
         get_level!(perm_v!(1)),
         fact![get_var_in_fact!(perm_v!(3), 1),
               get_list!(Level::Shallow, temp_v!(2)),
               unify_variable!(temp_v!(2)),
               unify_variable!(perm_v!(4))],
         is_var!(perm_v!(4)),
         query![put_value!(perm_v!(3), 1),
                put_var!(perm_v!(2), 3)],
         functor_call!(),
         cut!(perm_v!(1)),
         query![put_unsafe_value!(4, 1),
                put_value!(perm_v!(3), 2),
                put_constant!(Level::Shallow, integer!(1), temp_v!(3)),
                put_unsafe_value!(2, 4)],
         deallocate!(),
         goto_execute!(252, 4), // goto get_args/4, 239.
         trust_me!(),
         allocate!(5),
         get_level!(perm_v!(1)),
         fact![get_var_in_fact!(perm_v!(3), 1),
               get_list!(Level::Shallow, temp_v!(2)),
               unify_variable!(perm_v!(5)),
               unify_variable!(perm_v!(4))],
         query![put_value!(perm_v!(4), 1),
                put_var!(perm_v!(2), 2)],
         goto_call!(277, 2), // goto length/2, 245.
         query![put_value!(perm_v!(3), 1),
                put_value!(perm_v!(5), 2),
                put_value!(perm_v!(2), 3)],
         functor_call!(),
         cut!(perm_v!(1)),
         query![put_unsafe_value!(4, 1),
                put_value!(perm_v!(3), 2),
                put_constant!(Level::Shallow, integer!(1), temp_v!(3)),
                put_unsafe_value!(2, 4)],
         deallocate!(),
         goto_execute!(252, 4), // goto get_args/4, 251.
         try_me_else!(5), // get_args/4, 252.
         fact![get_var_in_fact!(temp_v!(5), 1),
               get_constant!(integer!(0), temp_v!(4))],
         neck_cut!(),
         query![put_value!(temp_v!(5), 1),
                put_constant!(Level::Shallow, Constant::EmptyList, temp_v!(2))],
         goto_execute!(73, 2), // goto =/2, 256.
         trust_me!(),
         switch_on_term!(3, 0, 1, 0),
         indexed_try!(3),
         trust!(7),
         try_me_else!(5),
         fact![get_list!(Level::Shallow, temp_v!(1)),
               unify_variable!(temp_v!(5)),
               unify_constant!(Constant::EmptyList),
               get_var_in_fact!(temp_v!(6), 2),
               get_var_in_fact!(temp_v!(1), 3),
               get_value!(temp_v!(1), 4)],
         neck_cut!(),
         query![put_value!(temp_v!(6), 2),
                put_value!(temp_v!(5), 3)],
         get_arg_execute!(),
         trust_me!(),
         allocate!(5),
         fact![get_list!(Level::Shallow, temp_v!(1)),
               unify_variable!(temp_v!(5)),
               unify_variable!(perm_v!(4)),
               get_var_in_fact!(perm_v!(3), 2),
               get_var_in_fact!(perm_v!(5), 3),
               get_var_in_fact!(perm_v!(1), 4)],
         query![put_value!(perm_v!(5), 1),
                put_value!(perm_v!(3), 2),
                put_value!(temp_v!(5), 3)],
         get_arg_call!(),
         add!(ArithmeticTerm::Reg(perm_v!(5)),
              ArithmeticTerm::Number(rc_integer!(1)),
              1),
         query![put_var!(perm_v!(2), 1)],
         is_call!(perm_v!(2), ArithmeticTerm::Interm(1)),
         query![put_unsafe_value!(4, 1),
                put_value!(perm_v!(3), 2),
                put_unsafe_value!(2, 3),
                put_value!(perm_v!(1), 4)],
         deallocate!(),
         goto_execute!(252, 4), // goto get_args/4, 276.
         try_me_else!(6), // length/2, 277.
         fact![get_var_in_fact!(temp_v!(4), 1),
               get_var_in_fact!(temp_v!(3), 2)],
         is_var!(temp_v!(3)),
         neck_cut!(),
         query![put_value!(temp_v!(4), 1),
                put_constant!(Level::Shallow, integer!(0), temp_v!(2))],
         goto_execute!(297, 3), // goto length/3, 282.
         retry_me_else!(10),
         allocate!(1),
         get_level!(perm_v!(1)),
         fact![get_var_in_fact!(temp_v!(4), 1),
               get_var_in_fact!(temp_v!(3), 2)],
         is_integer!(temp_v!(3)),
         query![put_value!(temp_v!(4), 1),
                put_constant!(Level::Shallow, integer!(0), temp_v!(2))],
         goto_call!(297, 3), // goto length/3, 289.
         cut!(perm_v!(1)),
         deallocate!(),
         proceed!(),
         trust_me!(),
         fact![get_var_in_fact!(temp_v!(3), 1),
               get_var_in_fact!(temp_v!(4), 2)],
         query![put_structure!("type_error", 2, temp_v!(1), None),
                set_constant!(atom!("integer_expected")),
                set_value!(temp_v!(4))],
         goto_execute!(59, 1), // goto throw/1, 296.
         switch_on_term!(1, 2, 5, 0), // length/3, 297.
         try_me_else!(3),
         fact![get_constant!(Constant::EmptyList, temp_v!(1)),
               get_var_in_fact!(temp_v!(4), 2),
               get_value!(temp_v!(4), 3)],
         proceed!(),
         trust_me!(),
         allocate!(3),
         fact![get_list!(Level::Shallow, temp_v!(1)),
               unify_void!(1),
               unify_variable!(perm_v!(1)),
               get_var_in_fact!(temp_v!(4), 2),
               get_var_in_fact!(perm_v!(3), 3)],
         add!(ArithmeticTerm::Reg(temp_v!(4)),
              ArithmeticTerm::Number(rc_integer!(1)),
              1),
         query![put_var!(perm_v!(2), 1)],
         is_call!(perm_v!(2), ArithmeticTerm::Interm(1)),
         query![put_unsafe_value!(1, 1),
                put_unsafe_value!(2, 2),
                put_value!(perm_v!(3), 3)],
         deallocate!(),
         goto_execute!(297, 3), // goto length/3, 309.
         allocate!(4), // setup_call_cleanup/3, 310.
         get_level!(perm_v!(1)),
         fact![get_var_in_fact!(perm_v!(2), 2),
               get_var_in_fact!(perm_v!(3), 3)],
         call_n!(1),
         cut!(perm_v!(1)),
         query![put_var!(perm_v!(4), 1)],
         get_current_block!(),
         query![put_value!(perm_v!(3), 1),
                put_unsafe_value!(4, 2),
                put_value!(perm_v!(2), 3)],
         deallocate!(),
         jmp_execute!(3, 1, 0),
         try_me_else!(5), // 320.
         is_var!(temp_v!(1)),
         neck_cut!(),
         query![put_constant!(Level::Shallow,
                              atom!("instantiation_error"),
                              temp_v!(1))],
         goto_execute!(59, 1),
         default_trust_me!(),
         query![get_var_in_query!(temp_v!(4), 2),
                put_value!(temp_v!(3), 2),
                get_var_in_query!(temp_v!(5), 3),
                put_value!(temp_v!(4), 3)],
         goto_execute!(328, 3),
         try_me_else!(13), // sgc_helper/3, 328.
         allocate!(4),
         fact![get_var_in_fact!(perm_v!(4), 1),
               get_var_in_fact!(perm_v!(3), 2),
               get_var_in_fact!(perm_v!(2), 3)],
         get_level!(perm_v!(1)),
         query![put_value!(perm_v!(4), 1)],
         install_cleaner!(),
         query![put_var!(temp_v!(2), 1)],
         install_new_block!(),
         query![put_value!(perm_v!(3), 1)],
         call_n!(1),
         query![put_value!(perm_v!(2), 1),
                put_value!(perm_v!(1), 2)],
         deallocate!(),
         check_cp_execute!(),
         default_retry_me_else!(12),
         allocate!(2),
         query![put_value!(temp_v!(3), 1)],
         reset_block!(),
         query![put_var!(perm_v!(1), 1)],
         get_ball!(),
         get_level!(perm_v!(2)),
         default_set_cp!(perm_v!(2)),
         goto_call!(358, 0), // goto run_cleaners_with_handling/0, 349.
         query![put_unsafe_value!(1, 1)],
         deallocate!(),
         goto_execute!(59, 1), // goto throw/1, 59.
         default_trust_me!(),
         allocate!(0),
         goto_call!(370, 0), // goto run_cleaners_without_handling/0, 355.
         deallocate!(),
         fail!(),
         try_me_else!(10), // run_cleaners_with_handling/0, 358.
         allocate!(2),
         get_level!(perm_v!(1)),
         query![put_var!(perm_v!(2), 1)],
         get_cleaner_call!(),
         query![put_value!(perm_v!(2), 1),
                put_var!(temp_v!(4), 2),
                put_constant!(Level::Shallow, atom!("true"), temp_v!(3))],
         goto_call!(5, 3), // goto catch/3, 5.
         default_set_cp!(perm_v!(1)),
         deallocate!(),
         goto_execute!(358, 0), // goto run_cleaners_with_handling/0, 367.
         default_trust_me!(),
         goto_execute!(398, 0), // goto restore_cut_points/0, 369.
         try_me_else!(10), // run_cleaners_without_handling/0, 370.
         allocate!(2),
         get_level!(perm_v!(1)),
         query![put_var!(perm_v!(2), 1)],
         get_cleaner_call!(),
         query![put_value!(perm_v!(2), 1)],
         call_n!(1),
         default_set_cp!(perm_v!(1)),
         deallocate!(),
         goto_execute!(370, 0), // goto run_cleaners_without_handling/0, 379.
         default_trust_me!(),
         goto_execute!(398, 0), // goto restore_cut_points/0, 381.
         allocate!(1), // sgc_on_success/2, 382.
         fact![get_var_in_fact!(perm_v!(1), 2)],
         reset_block!(),
         cut!(perm_v!(1)),
         deallocate!(),
         proceed!(),
         is_compound!(temp_v!(1)), // compound/1, 388.
         proceed!(),
         is_rational!(temp_v!(1)), // rational/1, 390.
         proceed!(),
         is_string!(temp_v!(1)), // string/1, 392.
         proceed!(),
         is_float!(temp_v!(1)), // float/1, 394.
         proceed!(),
         is_nonvar!(temp_v!(1)), // nonvar/1, 396.
         proceed!(),
         restore_cut_policy!(), // restore_cut_policy/0, 398.
         proceed!(),
         ground_execute!(), // ground/1, 400.
         eq_execute!(), // (==)/2, 401.
         not_eq_execute!(), // (\==)/2, 402.
         compare_term_execute!(term_cmp_gte!()), // (@>=)/2, 403.
         compare_term_execute!(term_cmp_lte!()), // (@=<)/2, 404.
         compare_term_execute!(term_cmp_gt!()), // (@>)/2, 405.
         compare_term_execute!(term_cmp_lt!()), // (@<)/2, 406.
         compare_term_execute!(term_cmp_eq!()), // (=@=)/2, 407.
         compare_term_execute!(term_cmp_ne!()), // (\=@=)/2, 408.
         allocate!(5), // call_with_inference_limit/3, 409.
         fact![get_var_in_fact!(perm_v!(4), 1),
               get_var_in_fact!(perm_v!(3), 2),
               get_var_in_fact!(perm_v!(2), 3)],
         query![put_var!(perm_v!(5), 1)],
         get_current_block!(),
         get_cp!(perm_v!(1)),
         query![put_value!(perm_v!(4), 1),
                put_value!(perm_v!(3), 2),
                put_value!(perm_v!(2), 3),
                put_value!(perm_v!(5), 4),
                put_value!(perm_v!(1), 5)],
         goto_call!(420, 5), // goto call_with_inference_limit/5, 415
         query![put_value!(perm_v!(1), 1)],
         deallocate!(),
         remove_call_policy_check!(),
         proceed!(),
         try_me_else!(19), // call_with_inference_limit/5, 420.
         allocate!(9),
         fact![get_var_in_fact!(perm_v!(9), 1),
               get_var_in_fact!(perm_v!(5), 2),
               get_var_in_fact!(perm_v!(8), 3),
               get_var_in_fact!(perm_v!(3), 4),
               get_var_in_fact!(perm_v!(4), 5)],
         query![put_var!(perm_v!(1), 1)],
         install_new_block!(),
         query![put_var!(perm_v!(7), 3)],
         install_inference_counter!(perm_v!(4), perm_v!(5), perm_v!(7)),
         query![put_value!(perm_v!(9), 1)],
         call_n!(1),
         inference_level!(perm_v!(8), perm_v!(4)),
         query![put_var!(perm_v!(6), 2)],
         remove_inference_counter!(perm_v!(4), perm_v!(6)),
         sub!(ArithmeticTerm::Reg(perm_v!(6)),
              ArithmeticTerm::Reg(perm_v!(7)),
              1),
         sub!(ArithmeticTerm::Reg(perm_v!(5)),
              ArithmeticTerm::Interm(1),
              1),
         query![put_var!(perm_v!(2), 1)],
         is_call!(temp_v!(1), ArithmeticTerm::Interm(1)),
         query![put_value!(perm_v!(4), 1),
                put_value!(perm_v!(3), 2),
                put_value!(perm_v!(1), 3),
                put_value!(perm_v!(2), 4)],
         deallocate!(),
         goto_execute!(468, 4), // goto end_block/4, 468
         default_trust_me!(), // 439
         allocate!(3),
         fact![get_var_in_fact!(perm_v!(1), 3),
               get_var_in_fact!(perm_v!(3), 5)],
         query![put_value!(temp_v!(4), 1)],
         reset_block!(),
         query![put_var!(temp_v!(3), 2)],
         remove_inference_counter!(perm_v!(3), temp_v!(2)),
         query![put_value!(perm_v!(3), 1),
                put_var!(perm_v!(2), 2)],
         jmp_call!(2, 5, 0),
         erase_ball!(),
         query![put_value!(perm_v!(3), 1),
                put_unsafe_value!(2, 2),
                put_value!(perm_v!(1), 3)],
         deallocate!(),
         goto_execute!(460, 3), // goto handle_ile/3, 451.
         try_me_else!(5), // the inner clause.
         query![put_value!(temp_v!(2), 1)],
         get_ball!(),
         neck_cut!(),
         proceed!(),
         default_trust_me!(),
         remove_call_policy_check!(),
         fail!(),
         try_me_else!(4), // handle_ile/3, 460.
         fact![get_structure!("inference_limit_exceeded", 1, temp_v!(2), None),
               unify_value!(temp_v!(1)),
               get_constant!(atom!("inference_limit_exceeded"), temp_v!(3))],
         neck_cut!(),
         proceed!(),
         default_trust_me!(),
         remove_call_policy_check!(),
         query![put_value!(temp_v!(2), 1)],
         goto_execute!(59, 1), // goto throw/1, 59.
         try_me_else!(6), // end_block/4, 468.
         query![put_value!(temp_v!(3), 1)],
         clean_up_block!(),
         query![put_value!(temp_v!(2), 1)],
         reset_block!(),
         proceed!(),
         default_trust_me!(),
         query![get_var_in_query!(temp_v!(5), 3),
                put_value!(temp_v!(4), 2),
                put_var!(temp_v!(6), 3)],
         install_inference_counter!(temp_v!(1), temp_v!(4), temp_v!(6)),
         query![put_value!(temp_v!(5), 1)],
         reset_block!(),
         fail!(),
         compare_execute!(), // compare/3, 480.
         is_atom!(temp_v!(1)), // atom/1, 481.
         proceed!(),
         sort_execute!(), // sort/2, 483.
         keysort_execute!(), // keysort/2, 484.
         acyclic_term_execute!(), // acyclic_term/1, 485.
         cyclic_term_execute!(), // cyclic_term/1, 486.
    ]
}

pub fn build_code_and_op_dirs() -> (CodeDir, OpDir)
{
    let mut code_dir = HashMap::new();
    let mut op_dir   = HashMap::new();

    let builtin = ClauseName::BuiltIn("builtin");

    op_dir.insert((clause_name!(":-"), Fixity::In),   (XFX, 1200, builtin.clone()));
    op_dir.insert((clause_name!(":-"), Fixity::Pre),  (FX, 1200, builtin.clone()));
    op_dir.insert((clause_name!("?-"), Fixity::Pre),  (FX, 1200, builtin.clone()));

    // control operators.
    op_dir.insert((clause_name!("\\+"), Fixity::Pre), (FY, 900, builtin.clone()));
    op_dir.insert((clause_name!("="), Fixity::In), (XFX, 700, builtin.clone()));

    // arithmetic operators.
    op_dir.insert((clause_name!("is"), Fixity::In), (XFX, 700, builtin.clone()));
    op_dir.insert((clause_name!("+"), Fixity::In), (YFX, 500, builtin.clone()));
    op_dir.insert((clause_name!("-"), Fixity::In), (YFX, 500, builtin.clone()));
    op_dir.insert((clause_name!("/\\"), Fixity::In), (YFX, 500, builtin.clone()));
    op_dir.insert((clause_name!("\\/"), Fixity::In), (YFX, 500, builtin.clone()));
    op_dir.insert((clause_name!("xor"), Fixity::In), (YFX, 500, builtin.clone()));
    op_dir.insert((clause_name!("//"), Fixity::In), (YFX, 400, builtin.clone()));
    op_dir.insert((clause_name!("/"), Fixity::In), (YFX, 400, builtin.clone()));
    op_dir.insert((clause_name!("div"), Fixity::In), (YFX, 400, builtin.clone()));
    op_dir.insert((clause_name!("*"), Fixity::In), (YFX, 400, builtin.clone()));
    op_dir.insert((clause_name!("-"), Fixity::Pre), (FY, 200, builtin.clone()));
    op_dir.insert((clause_name!("rdiv"), Fixity::In), (YFX, 400, builtin.clone()));
    op_dir.insert((clause_name!("<<"), Fixity::In), (YFX, 400, builtin.clone()));
    op_dir.insert((clause_name!(">>"), Fixity::In), (YFX, 400, builtin.clone()));
    op_dir.insert((clause_name!("mod"), Fixity::In), (YFX, 400, builtin.clone()));
    op_dir.insert((clause_name!("rem"), Fixity::In), (YFX, 400, builtin.clone()));

    // arithmetic comparison operators.
    op_dir.insert((clause_name!(">"), Fixity::In), (XFX, 700, builtin.clone()));
    op_dir.insert((clause_name!("<"), Fixity::In), (XFX, 700, builtin.clone()));
    op_dir.insert((clause_name!("=\\="), Fixity::In), (XFX, 700, builtin.clone()));
    op_dir.insert((clause_name!("=:="), Fixity::In), (XFX, 700, builtin.clone()));
    op_dir.insert((clause_name!(">="), Fixity::In), (XFX, 700, builtin.clone()));
    op_dir.insert((clause_name!("=<"), Fixity::In), (XFX, 700, builtin.clone()));

    // control operators.
    op_dir.insert((clause_name!(";"), Fixity::In), (XFY, 1100, builtin.clone()));
    op_dir.insert((clause_name!("->"), Fixity::In), (XFY, 1050, builtin.clone()));

    op_dir.insert((clause_name!("=.."), Fixity::In), (XFX, 700, builtin.clone()));
    op_dir.insert((clause_name!("=="), Fixity::In), (XFX, 700, builtin.clone()));
    op_dir.insert((clause_name!("\\=="), Fixity::In), (XFX, 700, builtin.clone()));
    op_dir.insert((clause_name!("@=<"), Fixity::In), (XFX, 700, builtin.clone()));
    op_dir.insert((clause_name!("@>="), Fixity::In), (XFX, 700, builtin.clone()));
    op_dir.insert((clause_name!("@<"), Fixity::In), (XFX, 700, builtin.clone()));
    op_dir.insert((clause_name!("@>"), Fixity::In), (XFX, 700, builtin.clone()));
    op_dir.insert((clause_name!("=@="), Fixity::In), (XFX, 700, builtin.clone()));
    op_dir.insert((clause_name!("\\=@="), Fixity::In), (XFX, 700, builtin.clone()));

    // there are 63 registers in the VM, so call/N is defined for all 0 <= N <= 62
    // (an extra register is needed for the predicate name)
    for arity in 0 .. 63 {
        code_dir.insert((clause_name!("call"), arity), CodeIndex::from((0, builtin.clone())));
    }

    code_dir.insert((clause_name!("atomic"), 1), CodeIndex::from((1, builtin.clone())));
    code_dir.insert((clause_name!("var"), 1), CodeIndex::from((3, builtin.clone())));
    code_dir.insert((clause_name!("false"), 0), CodeIndex::from((61, builtin.clone())));
    code_dir.insert((clause_name!("\\+"), 1), CodeIndex::from((62, builtin.clone())));
    code_dir.insert((clause_name!("duplicate_term"), 2), CodeIndex::from((71, builtin.clone())));
    code_dir.insert((clause_name!("catch"), 3), CodeIndex::from((5, builtin.clone())));
    code_dir.insert((clause_name!("throw"), 1), CodeIndex::from((59, builtin.clone())));
    code_dir.insert((clause_name!("="), 2), CodeIndex::from((73, builtin.clone())));
    code_dir.insert((clause_name!("true"), 0), CodeIndex::from((75, builtin.clone())));

    code_dir.insert((clause_name!(","), 2), CodeIndex::from((76, builtin.clone())));
    code_dir.insert((clause_name!(";"), 2), CodeIndex::from((120, builtin.clone())));
    code_dir.insert((clause_name!("->"), 2), CodeIndex::from((146, builtin.clone())));

    code_dir.insert((clause_name!("functor"), 3), CodeIndex::from((162, builtin.clone())));
    code_dir.insert((clause_name!("arg"), 3), CodeIndex::from((166, builtin.clone())));
    code_dir.insert((clause_name!("integer"), 1), CodeIndex::from((163, builtin.clone())));
    code_dir.insert((clause_name!("display"), 1), CodeIndex::from((208, builtin.clone())));

    code_dir.insert((clause_name!("is"), 2), CodeIndex::from((210, builtin.clone())));
    code_dir.insert((clause_name!(">"), 2), CodeIndex::from((212, builtin.clone())));
    code_dir.insert((clause_name!("<"), 2), CodeIndex::from((214, builtin.clone())));
    code_dir.insert((clause_name!(">="), 2), CodeIndex::from((216, builtin.clone())));
    code_dir.insert((clause_name!("=<"), 2), CodeIndex::from((218, builtin.clone())));
    code_dir.insert((clause_name!("=\\="), 2), CodeIndex::from((220, builtin.clone())));
    code_dir.insert((clause_name!("=:="), 2), CodeIndex::from((222, builtin.clone())));
    code_dir.insert((clause_name!("=.."), 2), CodeIndex::from((224, builtin.clone())));

    code_dir.insert((clause_name!("length"), 2), CodeIndex::from((277, builtin.clone())));
    code_dir.insert((clause_name!("setup_call_cleanup"), 3),
                    CodeIndex::from((310, builtin.clone())));
    code_dir.insert((clause_name!("call_with_inference_limit"), 3),
                    CodeIndex::from((409, builtin.clone())));

    code_dir.insert((clause_name!("compound"), 1), CodeIndex::from((388, builtin.clone())));
    code_dir.insert((clause_name!("rational"), 1), CodeIndex::from((390, builtin.clone())));
    code_dir.insert((clause_name!("string"), 1), CodeIndex::from((392, builtin.clone())));
    code_dir.insert((clause_name!("float"), 1), CodeIndex::from((394, builtin.clone())));
    code_dir.insert((clause_name!("nonvar"), 1), CodeIndex::from((396, builtin.clone())));

    code_dir.insert((clause_name!("ground"), 1), CodeIndex::from((400, builtin.clone())));
    code_dir.insert((clause_name!("=="), 2), CodeIndex::from((401, builtin.clone())));
    code_dir.insert((clause_name!("\\=="), 2), CodeIndex::from((402, builtin.clone())));
    code_dir.insert((clause_name!("@>="), 2), CodeIndex::from((403, builtin.clone())));
    code_dir.insert((clause_name!("@=<"), 2), CodeIndex::from((404, builtin.clone())));
    code_dir.insert((clause_name!("@>"), 2), CodeIndex::from((405, builtin.clone())));
    code_dir.insert((clause_name!("@<"), 2), CodeIndex::from((406, builtin.clone())));
    code_dir.insert((clause_name!("=@="), 2), CodeIndex::from((407, builtin.clone())));
    code_dir.insert((clause_name!("\\=@="), 2), CodeIndex::from((408, builtin.clone())));
    code_dir.insert((clause_name!("compare"), 3), CodeIndex::from((480, builtin.clone())));
    code_dir.insert((clause_name!("atom"), 1), CodeIndex::from((481, builtin.clone())));
    code_dir.insert((clause_name!("sort"), 2), CodeIndex::from((483, builtin.clone())));
    code_dir.insert((clause_name!("keysort"), 2), CodeIndex::from((484, builtin.clone())));
    code_dir.insert((clause_name!("acyclic_term"), 1), CodeIndex::from((485, builtin.clone())));
    code_dir.insert((clause_name!("cyclic_term"), 1), CodeIndex::from((486, builtin.clone())));

    (code_dir, op_dir)
}

pub fn default_build() -> (Code, CodeDir, OpDir)
{
    let builtin_code = get_builtins();
    let (code_dir, op_dir) = build_code_and_op_dirs();

    (builtin_code, code_dir, op_dir)
}

#[allow(dead_code)]
pub fn builtin_module() -> Module
{
    let (code_dir, op_dir) = build_code_and_op_dirs();
    let mut module_decl = module_decl!(clause_name!("builtin"),
                                       vec![(clause_name!("atomic"), 1),
                                            (clause_name!("var"), 1),
                                            (clause_name!("false"), 0),
                                            (clause_name!("catch"), 3),
                                            (clause_name!("throw"), 1),
                                            (clause_name!("(\\+)"), 1),
                                            (clause_name!("duplicate_term"), 2),
                                            (clause_name!("(=)"), 2),
                                            (clause_name!("true"), 0),
                                            (clause_name!("(,)"), 2),
                                            (clause_name!("(;)"), 2),
                                            (clause_name!("->"), 2),
                                            (clause_name!("functor"), 3),
                                            (clause_name!("arg"), 3),
                                            (clause_name!("(=..)"), 3),
                                            (clause_name!("display"), 1),
                                            (clause_name!("is"), 2),
                                            (clause_name!("(>)"), 2),
                                            (clause_name!("(<)"), 2),
                                            (clause_name!("(>=)"), 2),
                                            (clause_name!("(=<)"), 2),
                                            (clause_name!("(=\\=)"), 2),
                                            (clause_name!("(=:=)"), 2),
                                            (clause_name!("(@>)"), 2),
                                            (clause_name!("(@<)"), 2),
                                            (clause_name!("(@>=)"), 2),
                                            (clause_name!("(@=<)"), 2),
                                            (clause_name!("(=@=)"), 2),
                                            (clause_name!("(\\=@=)"), 2),
                                            (clause_name!("(==)"), 2),
                                            (clause_name!("(\\==)"), 2),
                                            (clause_name!("length"), 2),
                                            (clause_name!("compound"), 1),
                                            (clause_name!("rational"), 1),
                                            (clause_name!("integer"), 1),
                                            (clause_name!("string"), 1),
                                            (clause_name!("float"), 1),
                                            (clause_name!("nonvar"), 1),
                                            (clause_name!("ground"), 1),
                                            (clause_name!("setup_call_cleanup"), 3),
                                            (clause_name!("call_with_inference_limit"), 3),
                                            (clause_name!("compare"), 3),
                                            (clause_name!("atom"), 1),
                                            (clause_name!("sort"), 2),
                                            (clause_name!("keysort"), 2),
                                            (clause_name!("acyclic_term"), 1),
                                            (clause_name!("cyclic_term"), 1)]);

    for arity in 0 .. 63 {
        module_decl.exports.push((clause_name!("call"), arity));
    }

    Module { module_decl, code_dir, op_dir }
}
