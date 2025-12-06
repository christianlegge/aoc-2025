use std::str::FromStr;

use anyhow::Error;
use itertools::Itertools;

struct Operation {
    operands: Vec<u64>,
    operator: Operator,
}

impl Operation {
    pub fn evaluate(&self) -> u64 {
        match self.operator {
            Operator::Add => self.operands.iter().sum(),
            Operator::Multiply => self.operands.iter().product(),
        }
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl FromStr for Operator {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            _ => Err(anyhow::anyhow!("unexpected string {s}")),
        }
    }
}

fn collect_operands(s: &[&str]) -> Result<Vec<Vec<u64>>, Error> {
    let mut result = Vec::new();
    for &line in s {
        let nums = line.split_whitespace();
        let vec: Vec<u64> = nums.map(str::parse::<u64>).try_collect()?;
        result.push(vec);
    }
    Ok(result)
}

fn transpose_vector<T: Clone>(v: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let inner_len = v.first().expect("vector is not empty").len();
    for i in 0..inner_len {
        let mut inner = Vec::new();
        for ele in v {
            let inner_ele = ele.get(i).expect("missing element");
            inner.push((*inner_ele).clone());
        }
        result.push(inner);
    }
    result
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve(data: &str) -> Result<(String, String), Error> {
    let lines = data.lines().collect::<Vec<_>>();
    let Some((&operators, operands)) = lines.split_last() else {
        return Err(anyhow::anyhow!("error splitting input"));
    };

    let operands = collect_operands(operands)?;
    let operands = transpose_vector(&operands);

    let operators: Vec<Operator> = operators
        .split_whitespace()
        .map(Operator::from_str)
        .try_collect()?;

    let mut operations = Vec::new();

    for (a, b) in operands.iter().zip(operators) {
        let operation = Operation {
            operands: a.clone(),
            operator: b,
        };
        operations.push(operation);
    }

    let char_vecs = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut transposed_input = transpose_vector(&char_vecs);
    transposed_input.push(vec![' ']);
    let mut iter = transposed_input.iter().peekable();

    let mut operations2 = Vec::new();

    while iter.peek().is_some() {
        let mut nums = Vec::new();
        let mut operator = None;
        loop {
            let line = iter.next().expect("end of iterator");
            let Some((c, rest)) = line.split_last() else {
                return Err(anyhow::anyhow!("error splitting {line:?}"));
            };

            if line.iter().all(|&c| c == ' ') {
                operations2.push(Operation {
                    operands: nums,
                    operator: operator.expect("operator not found"),
                });
                break;
            }
            if *c != ' ' {
                operator = Some(Operator::from_str(&c.to_string())?);
            }
            nums.push(rest.iter().collect::<String>().trim().parse()?);
        }
    }

    Ok((
        operations
            .iter()
            .map(Operation::evaluate)
            .sum::<u64>()
            .to_string(),
        operations2
            .iter()
            .map(Operation::evaluate)
            .sum::<u64>()
            .to_string(),
    ))
}
