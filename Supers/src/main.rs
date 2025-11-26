// main.rs

use crate::core::{IData, IAttribute, IVariable, Variable};

/// Simple type-tag enum for the u32 attribute slot.
/// We use #[repr(u32)] so we can cast to/from u32 directly.
#[repr(u32)]
enum ValueKind {
    String = 1,
    Int = 2,
    Pointer = 3,
}

impl ValueKind {
    fn from_u32(raw: u32) -> Option<Self> {
        match raw {
            1 => Some(ValueKind::String),
            2 => Some(ValueKind::Int),
            3 => Some(ValueKind::Pointer),
            _ => None,
        }
    }
}

/// Small helper to debug the “type” of a Variable
/// based on its u32 attribute slot and label string.
fn debug_variable_type<D, V>(v: &V)
where
    V: IData<D> + IAttribute<u32>, // i.e. IVariable<D, u32>
{
    let label = v.label();
    let tag = *v.attribute();

    let kind = ValueKind::from_u32(tag);
    match kind {
        Some(ValueKind::String) => {
            println!("[TYPE] {label} => String (tag={tag})");
        }
        Some(ValueKind::Int) => {
            println!("[TYPE] {label} => Int (tag={tag})");
        }
        Some(ValueKind::Pointer) => {
            println!("[TYPE] {label} => Pointer/Index (tag={tag})");
        }
        None => {
            println!("[TYPE] {label} => Unknown (raw tag={tag})");
        }
    }
}

fn main() {
    // ──────────────────────────────────────────────
    // 1. Read the command-line argument
    // ──────────────────────────────────────────────
    let mut args = std::env::args();
    let _exe_name = args.next(); // ignore program path

    let command_input = args
        .next()
        .unwrap_or_else(|| "say-hello".to_string());

    // ──────────────────────────────────────────────
    // 2. Create a Variable<String> for the COMMAND
    //    Data type (D) = String
    //    Attribute type (A) = u32 → ValueKind::String
    // ──────────────────────────────────────────────
    let mut command: Variable<String> = Variable::new(
        "command:String",              // label/meta
        command_input,                 // data payload
        ValueKind::String as u32,      // type tag in the u32 slot
    );

    // ──────────────────────────────────────────────
    // 3. Create a Variable<String> for the MESSAGE
    //    (also tagged as String in its attribute slot)
    // ──────────────────────────────────────────────
    let mut message: Variable<String> = Variable::new(
        "message:String",              // label/meta
        String::new(),                 // data payload
        ValueKind::String as u32,      // type tag
    );

    // ──────────────────────────────────────────────
    // 4. Also create a Variable<i32> for a NUMBER
    //    to show the same pattern with a different T
    //    but the same fixed attribute type (u32)
    // ──────────────────────────────────────────────
    let mut number: Variable<i32> = Variable::new(
        "number:Int",                  // label/meta
        0,                             // data payload (i32)
        ValueKind::Int as u32,         // type tag
    );

    // We can store some value in the number variable:
    number.set_data(42);

    // ──────────────────────────────────────────────
    // 5. Interpret the command and set message.data
    // ──────────────────────────────────────────────
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
            // Here we just hardcode 123 as some imaginary index.
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

    // ──────────────────────────────────────────────
    // 6. Show how we can query the type of each variable
    //    based on its attribute slot (u32) + label string.
    // ──────────────────────────────────────────────
    debug_variable_type(&command);
    debug_variable_type(&message);
    debug_variable_type(&number);

    // ──────────────────────────────────────────────
    // 7. Print the final message to the console
    // ──────────────────────────────────────────────
    println!(
        "[{}] {}",
        message.label(),
        message.get_data()
    );
}
