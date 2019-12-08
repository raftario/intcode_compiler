use crate::{error::Error, interpreter};

static MAIN: &str = include_str!("../resources/main.rs");
static ERROR: &str = include_str!("./error.rs");
static INTERPRETER: &str = include_str!("./interpreter.rs");

fn transpile_output(output: &[i64]) -> String {
    let output = output
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    format!("println!({:?});", output)
}

fn transpile_code(code: &[i64]) -> String {
    let len = code.len();
    format!("let mut code: [i64; {}] = {:?}", len, code)
}

fn transpile_iterator(i: usize) -> String {
    format!("let mut i: usize = {};", i)
}

pub fn transpile(code: Vec<i64>, input: Vec<i64>) -> Result<String, Error> {
    let mut result = MAIN.to_owned();
    let eval_results = interpreter::eval(code, input)?;

    if !eval_results.output.is_empty() {
        result = result.replace("// output", &transpile_output(&eval_results.output));
    } else {
        result = result.replace("    // output\n", "");
    }

    if eval_results.completed {
        result = result
            .replace("    // code\n", "")
            .replace("    // iterator\n", "");
        return Ok(result);
    }

    result = result
        .replace("// code", &transpile_code(&eval_results.code))
        .replace("// iterator", &transpile_iterator(eval_results.run_code));

    let mut inter = INTERPRETER.split('\n').collect::<Vec<&str>>();
    inter.truncate(343);
    inter.remove(0);
    let mut err = ERROR.to_owned();
    err.push('\n');
    err.push_str(&inter.join("\n"));
    err.push('\n');
    err.push_str(&result);
    result = err;

    Ok(result)
}
