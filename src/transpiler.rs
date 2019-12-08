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
    format!("let mut code: [i64; {}] = {:?};", len, code)
}

fn transpile_iterator(i: usize) -> String {
    format!("let i: usize = {};", i)
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
        let mut result: Vec<&str> = result.split('\n').collect();
        result.truncate(2);
        result.push("}\n");
        return Ok(result.join("\n"));
    }

    result = result
        .replace("// code", &transpile_code(&eval_results.code))
        .replace("// iterator", &transpile_iterator(eval_results.run_code));

    let mut inter: Vec<&str> = INTERPRETER.split('\n').collect();
    inter.truncate(342);
    inter.remove(0);
    let mut err = ERROR.to_owned();
    err.push('\n');
    err.push_str(&inter.join("\n"));
    err.push('\n');
    err.push_str(&result);
    result = err;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::transpiler::{transpile_code, transpile_iterator, transpile_output};

    #[test]
    fn output() {
        let output = vec![1, 2, 3];
        let expected = "println!(\"1\\n2\\n3\");".to_owned();
        assert_eq!(expected, transpile_output(&output));
    }

    #[test]
    fn code() {
        let code = vec![1, 2, 3];
        let expected = "let mut code: [i64; 3] = [1, 2, 3];".to_owned();
        assert_eq!(expected, transpile_code(&code));
    }

    #[test]
    fn iterator() {
        let i = 0;
        let expected = "let i: usize = 0;".to_owned();
        assert_eq!(expected, transpile_iterator(i));
    }
}
