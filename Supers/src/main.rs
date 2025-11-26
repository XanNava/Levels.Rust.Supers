// ─────────────────────────────────────────────────────────────────────────────
// main() – hello world with a command Variable and a message Variable
// ─────────────────────────────────────────────────────────────────────────────

use crate::core::{IData, IAttribute, Variable};

fn main() {
    // 1. Read command-line args: first arg after program name is our command.
    //
    //    Example:
    //       cargo run say-hello
    //
    //    - args[0] = path to executable
    //    - args[1] = "say-hello"
    let mut args = std::env::args();
    let _exe_name = args.next(); // we don't need the executable name

    let command_input = args.next().unwrap_or_else(|| "say-hello".to_string());

    // 2. Create a Variable<String> for the COMMAND.
    //
    //    - label:  "command"
    //    - value:  the string from the CLI
    let mut command: Variable<String> = Variable::new("command", command_input);

    // 3. Create a Variable<String> for the MESSAGE.
    //
    //    - label:  "message"
    //    - value:  starts empty; we'll fill it based on the command.
    let mut message: Variable<String> = Variable::new("message", String::new());

    // 4. Interpret the command and set the message accordingly.
    //
    //    Here we use IAttribute/IData methods instead of fields
    //    so you can see the trait semantics in action.

    match command.attribute().as_str() {
        "say-hello" => {
            // Set the message's attribute value.
            message.set_attribute("Hello, world!".to_string());
        }
        "shout" => {
            message.set_attribute("HELLO FROM VARIABLE!".to_string());
        }
        other => {
            // Unknown command -> we mention it in the message.
            let fallback = format!("Unknown command: '{}'", other);
            message.set_attribute(fallback);
        }
    }

    // 5. Optionally tweak labels if you want to see them:
    //
    //    (Just to show IAttribute’s label API.)
    command.set_label("CommandVariable");
    message.set_label("MessageVariable");

    // 6. Output to console.
    //
    //    We could use either:
    //      - message.attribute()  (attribute-level view)
    //      - message.get_data()   (data-level view)
    //
    //    Here I’ll use attribute(), since "message" is conceptually an attribute.
    println!(
        "[{}={}] -> {}",
        command.label(),
        command.attribute(),
        message.attribute()
    );
}
