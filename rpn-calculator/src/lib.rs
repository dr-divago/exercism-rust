#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();
    for value in inputs {
        match value {
            CalculatorInput::Value(x) => stack.push(*x),
            CalculatorInput::Add => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(a + b);
            },
            CalculatorInput::Subtract => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b - a);
            },
            CalculatorInput::Multiply => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(a * b);
            },
            CalculatorInput::Divide => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b / a);
            }
        }
    }

    match stack.len() {
        0 => None,
        1 => Some(stack.pop().unwrap()),
        _ => None
    }
}
