#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

use CalculatorInput::{Add,Subtract,Multiply,Divide,Value};

fn operation(stack: &mut Vec<i32>, input: &CalculatorInput) {
    let right = stack.pop().unwrap();
    let left = stack.pop().unwrap();
    stack.push(match input {
        Add => left + right,
        Subtract => left - right,
        Multiply => left * right,
        Divide => left / right,
        _ => unreachable!(),
    });
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();
    for input in inputs {
        match input {
            Value(value) => stack.push(*value),
            _ if stack.len() < 2 => return None,
            _ => operation(&mut stack, &input),
        }
    }
    if stack.len() == 1 {
        Some(stack[0])
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

		fn calculator_input(s: &str) -> Vec<CalculatorInput> {
				s.split_whitespace()
				    .map(|s| match s {
				        "+" => CalculatorInput::Add,
				        "-" => CalculatorInput::Subtract,
				        "*" => CalculatorInput::Multiply,
				        "/" => CalculatorInput::Divide,
				        n => CalculatorInput::Value(n.parse().unwrap()),
				    })
				    .collect()
		}

		#[test]
		fn test_empty_input_returns_none() {
				let input = calculator_input("");
				assert_eq!(evaluate(&input), None);
		}

		#[test]
		// #[ignore]
		fn test_simple_value() {
				let input = calculator_input("10");
				assert_eq!(evaluate(&input), Some(10));
		}

		#[test]
		// #[ignore]
		fn test_simple_addition() {
				let input = calculator_input("2 2 +");
				assert_eq!(evaluate(&input), Some(4));
		}

		#[test]
		// #[ignore]
		fn test_simple_subtraction() {
				let input = calculator_input("7 11 -");
				assert_eq!(evaluate(&input), Some(-4));
		}

		#[test]
		// #[ignore]
		fn test_simple_multiplication() {
				let input = calculator_input("6 9 *");
				assert_eq!(evaluate(&input), Some(54));
		}

		#[test]
		// #[ignore]
		fn test_simple_division() {
				let input = calculator_input("57 19 /");
				assert_eq!(evaluate(&input), Some(3));
		}

		#[test]
		// #[ignore]
		fn test_complex_operation() {
				let input = calculator_input("4 8 + 7 5 - /");
				assert_eq!(evaluate(&input), Some(6));
		}

		#[test]
		// #[ignore]
		fn test_too_few_operands_returns_none() {
				let input = calculator_input("2 +");
				assert_eq!(evaluate(&input), None);
		}

		#[test]
		// #[ignore]
		fn test_too_many_operands_returns_none() {
				let input = calculator_input("2 2");
				assert_eq!(evaluate(&input), None);
		}

}
