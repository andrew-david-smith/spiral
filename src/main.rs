// use inkwell::context::Context;
use std::error::Error;
mod tokenizer;

fn main() -> Result<(), Box<dyn Error>> {
    let mut t = tokenizer::Tokenizer::build("'Really long unclosed string'");
    let tokens = t.execute()?;

    for token in tokens {
        println!("{:?} - {}", token.token_type, token.value);
    }

    Ok(())
    // let context = Context::create();
    // let module = context.create_module("test");
    // let builder = context.create_builder();

    // let i32_type = context.i32_type();
    // let i8_type = context.i8_type();
    // let i8_ptr_type = i8_type.ptr_type(inkwell::AddressSpace::default());
    // let printf_fn_type = i32_type.fn_type(&[i8_ptr_type.into()], true);
    // let printf_function = module.add_function("printf", printf_fn_type, None);

    // let fn_type = context.i32_type().fn_type(&[], false);
    // let fn_val = module.add_function("main", fn_type, None);
    // let basic_block = context.append_basic_block(fn_val, "entry");
    // builder.position_at_end(basic_block);
    // //let value = context.i32_type().const_int(52, false);
    // let value = builder.build_global_string_ptr("Hello World!\n", "test");

    // builder.build_call(printf_function, &[value.as_pointer_value().into()], "call");

    // builder.build_return(Some(&context.i32_type().const_zero()));

    // module.print_to_file("outrs.ll").ok();
    // Ok(())
}
