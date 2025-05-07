/// Copy of the nushell print command with a slight adjustment for pipelines
/// Source: https://github.com/nushell/nushell/blob/98525043edd20abb62da09726d75816d09d68f1e/crates/nu-cli/src/print.rs
use nu_engine::CallExt;
use nu_protocol::engine::{Call, Command, EngineState, Stack};
use nu_protocol::{
    Category, Example, PipelineData, ShellError, Signature,
    SyntaxShape, Type, Value, IntoPipelineData,Signals
};



#[derive(Clone)]
pub struct PrintCommand;

impl Command for PrintCommand {
    fn name(&self) -> &str {
        "print"
    }

    fn description(&self) -> &str {
        "Display values to stdout without consuming them (and pass input through)"
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Print a literal string",
                example: "print 'Hello, world!'",
                result: None,
            },
            Example {
                description: "Print each value in a list",
                example: "print 1 2 3",
                result: None,
            },
        ]
    }

    fn signature(&self) -> Signature {
        Signature::build("print")
            .input_output_types(vec![
                (Type::Nothing, Type::Nothing),
                (Type::Any, Type::Nothing),
            ])
            .allow_variants_without_examples(true)
            .rest("rest", SyntaxShape::Any, "the values to print")
            .switch(
                "no-newline",
                "print without inserting a newline for the line ending",
                Some('n'),
            )
            .switch("stderr", "print to stderr instead of stdout", Some('e'))
            .category(Category::Strings)
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        // 1) Collect explicit `print …` args
        let args: Vec<Value> = call.rest(engine_state, stack, 0)?;
        let no_newline = call.has_flag(engine_state, stack, "no-newline")?;
        let to_stderr = call.has_flag(engine_state, stack, "stderr")?;
    
        // 2) Turn incoming pipeline into a single Value, unwrapping the Result
        let input_val: Value = input.into_value(call.head)?;    // <-- note the `?`
    
        // We'll use the call's span and an "empty" Signals instance for all our into_pipeline_data calls
        let _span = call.head;
        let _signals = Signals::empty();
    
        // 3) Print either the explicit args…
        if !args.is_empty() {
            for arg in args {
                arg.into_pipeline_data()
                    .print_raw(engine_state, no_newline, to_stderr)?;
            }
        }
        // …or, if nothing was passed, print the incoming value (if it isn't "nothing")
        else if !input_val.is_nothing() {
            input_val.clone().into_pipeline_data().print_raw(
                engine_state,
                no_newline,
                to_stderr,
            )?;
        }
    
        // 4) Finally, send the original input right back down the pipeline
        Ok(input_val.into_pipeline_data())
    }
    
    
}
