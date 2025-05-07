use embed_nu::{rusty_value::*, NewEmpty};
use embed_nu::{CommandGroupConfig, Context, PipelineData};
use nu_protocol::{Span,};

#[test]
fn it_evals_strings() {
    let mut ctx = get_context();
    let pipeline = ctx
        .eval_raw(
            r#"echo "Hello World from this eval""#,
            PipelineData::empty(),
        )
        .unwrap();
    ctx.print_pipeline(pipeline).unwrap()
}

#[test]
fn it_evals_print() {
    let mut ctx = get_context();
    ctx.eval_raw(
        r#"print "Hello World from this eval using print""#,
        PipelineData::empty(),
    )
    .unwrap();
}

#[test]
fn it_reports_parse_errors() {
    let mut ctx = get_context();
    let eval_result = ctx.eval_raw(r#"let a = 1 || 2"#, PipelineData::empty());
    assert!(eval_result.is_err());
}

#[test]
fn it_returns_variables() {
    let mut ctx = get_context();
    ctx.eval_raw(r#"let hello = 'world'"#, PipelineData::empty())
        .unwrap();
    let val = ctx.get_var("hello").expect("No variable returned");
    assert_eq!(val.as_str().unwrap(), String::from("world"))
}

#[test]
fn it_accepts_variables() {
    let mut ctx = get_context();
    ctx.add_var("hello", "world").unwrap();

    let val = ctx.get_var("hello").expect("No variable returned");
    assert_eq!(val.as_str().unwrap(), String::from("world"));

    let val = ctx
        .eval_raw(r#"$hello"#, PipelineData::empty())
        .unwrap()
        .into_value(Span::empty())
        .unwrap();

    assert_eq!(val.coerce_str().unwrap(), String::from("world"))
}

#[derive(RustyValue)]
struct TestArg {
    foo: String,
    bar: usize,
}

#[test]
fn it_executes_functions() {
    let mut ctx = get_context();
    ctx.eval_raw(
        r#"
    
        def hello [] {
            echo "Hello World from this script";
            echo # dummy echo so I don't have to print the output
        }        
        
    "#,
        PipelineData::empty(),
    )
    .unwrap();
    ctx.call_fn("hello", [] as [String; 0]).unwrap();
    assert!(ctx.has_fn("world") == false);

    let test_arg = TestArg {
        foo: String::from("Hello World"),
        bar: 12,
    };
    let pipeline = ctx.call_fn("echo", [test_arg]).unwrap();
    ctx.print_pipeline(pipeline).unwrap();
}

fn get_context() -> Context {
    Context::builder()
        .with_command_groups(CommandGroupConfig::default().all_groups(true))
        .unwrap()
        .add_parent_env_vars()
        .build()
        .unwrap()
}
