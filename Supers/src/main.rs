// main.rs

use crate::core::{IData, IAttribute, IVariable, Variable};
use crate::variable_util::{ValueKind, debug_variable_type};

fn main() {
    // 1. Read the command-line argument
    let mut args = std::env::args();
    let _exe_name = args.next(); // ignore program path

    let command_input = args
        .next()
        .unwrap_or_else(|| "say-hello".to_string());

    // Variable<String> for the COMMAND (tagged as String)
    let mut command: Variable<String> = Variable::new(
        "command:String",
        command_input,
        ValueKind::String as u32,
    );

    // Variable<String> for the MESSAGE (also String)
    let mut message: Variable<String> = Variable::new(
        "message:String",
        String::new(),
        ValueKind::String as u32,
    );

    // Variable<i32> for a NUMBER (tagged as Int)
    let mut number: Variable<i32> = Variable::new(
        "number:Int",
        0,
        ValueKind::Int as u32,
    );

    number.set_data(42);

    // 2. Interpret the command and set message.data
    match command.data().as_str() {
        "say-hello" => {
            message.set_data("Hello, world!".to_string());
        }
        "show-number" => {
            let text = format!("The number is: {}", number.get_data());
            message.set_data(text);
        }
        "pointer-demo" => {
            // Example: treat the u32 slot as a “pointer/index” tag.
            number.set_attribute(123);
            number.set_label("number:PointerIndex");
            message.set_data(format!(
                "Pointer-like index stored in attribute: {}",
                number.attribute()
            ));
        }
        other => {
            message.set_data(format!("Unknown command: '{}'", other));
        }
    }

    // 3. Use the utility helper to introspect the type info
    debug_variable_type(&command);
    debug_variable_type(&message);
    debug_variable_type(&number);

    // 4. Print the final message to the console
    println!(
        "[{}] {}",
        message.label(),
        message.get_data()
    );
}
