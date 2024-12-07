#[derive(Debug)]
struct Expression {
    result: u64,
    operands: Vec<u64>,
}

impl Expression {
    fn calculate(&self, operators: &Vec<Ops>) -> u64 {
        let mut result = self.operands[0];
        for idx in 0..operators.len() {
            match operators[idx] {
                Ops::Add => result += self.operands[idx + 1],
                Ops::Multiply => result *= self.operands[idx + 1],
                Ops::Concat => {
                    result = format!("{result}{}", self.operands[idx + 1])
                        .parse()
                        .expect("Concat result must be parsable as a number")
                }
            }
        }
        result
    }
}

impl From<&str> for Expression {
    fn from(value: &str) -> Self {
        let mut parts = value.split(":");
        let result = parts
            .next()
            .expect("Expected result before :")
            .parse()
            .expect("Expected result to be number");
        let operands: Vec<u64> = parts
            .next()
            .expect("Expected operands after :")
            .split(" ")
            .filter_map(|op| op.parse().ok())
            .collect();
        return Self { result, operands };
    }
}

#[derive(Debug, Clone)]
enum Ops {
    Add,
    Multiply,
    Concat,
}

#[derive(Debug)]
struct OperatorGenerator {
    size: u32,
    base: usize,
    current_combination: usize,
}

impl OperatorGenerator {
    fn new(size: u32, base: usize) -> OperatorGenerator {
        OperatorGenerator {
            size,
            base,
            current_combination: 0,
        }
    }
}

impl Iterator for OperatorGenerator {
    type Item = Vec<Ops>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_combination == usize::pow(self.base, self.size) {
            None
        } else {
            let mut current = self.current_combination;
            self.current_combination += 1;
            let mut ops = vec![];
            for idx in 0..self.size {
                let remainder = current % self.base;
                current /= self.base;
                match remainder {
                    0 => ops.push(Ops::Add),
                    1 => ops.push(Ops::Multiply),
                    2 => ops.push(Ops::Concat),
                    _ => panic!("Don't support more than 3 operators"),
                }
            }
            Some(ops)
        }
    }
}

fn read_input() -> Vec<Expression> {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.into())
        .collect()
}

fn test_expression(exp: &Expression, with_concat: bool) -> bool {
    let generator = OperatorGenerator::new(
        exp.operands.len() as u32 - 1,
        if with_concat { 3 } else { 2 },
    );
    for operators in generator {
        if exp.calculate(&operators) == exp.result {
            return true;
        }
    }
    false
}

fn task1() {
    let expressions = read_input();

    let sum = expressions.iter().fold(0, |sum, exp| {
        if test_expression(exp, false) {
            sum + exp.result
        } else {
            sum
        }
    });

    println!("Task 1: Sum: {sum}");
}

fn task2() {
    let expressions = read_input();

    let sum = expressions.iter().fold(0, |sum, exp| {
        if test_expression(exp, true) {
            sum + exp.result
        } else {
            sum
        }
    });

    println!("Task 2: Sum: {sum}");
}

fn main() {
    task1();
    task2();
}
