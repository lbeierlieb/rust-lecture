// TODO: implement the type  Cpu
// The type must derive at least Debug.
#[derive(Debug)]
struct Cpu {
    stack: [u64; 5],
    stack_pointer: usize,
}

/// Return a fresh Cpu (with empty stack).
fn new_cpu() -> Cpu {
    Cpu {
        stack: [0; 5],
        stack_pointer: 0,
    }
}

/// Emulate the execution of the instruction on the cpu.
///
/// See the Instruction enum for how the instructions should modify the CPU state.
fn execute_instruction(mut cpu: Cpu, instruction: Instruction) -> InstructionExecutionResult {
    match instruction {
        Instruction::Push(val) => {
            if cpu.stack_pointer == 5 {
                InstructionExecutionResult::Error(InstructionExecutionError::StackOverflow)
            } else {
                cpu.stack[cpu.stack_pointer] = val;
                cpu.stack_pointer += 1;
                InstructionExecutionResult::SuccessWithoutOutput(cpu)
            }
        }
        Instruction::Pop => {
            if cpu.stack_pointer == 0 {
                InstructionExecutionResult::Error(InstructionExecutionError::StackUnderflow)
            } else {
                cpu.stack_pointer -= 1;
                let popped = cpu.stack[cpu.stack_pointer];
                InstructionExecutionResult::SuccessWithOutput(popped, cpu)
            }
        }
        Instruction::Add => {
            if cpu.stack_pointer < 2 {
                InstructionExecutionResult::Error(InstructionExecutionError::StackUnderflow)
            } else {
                cpu.stack[cpu.stack_pointer - 2] += cpu.stack[cpu.stack_pointer - 1];
                cpu.stack_pointer -= 1;
                InstructionExecutionResult::SuccessWithoutOutput(cpu)
            }
        }
        Instruction::Multiply => {
            if cpu.stack_pointer < 2 {
                InstructionExecutionResult::Error(InstructionExecutionError::StackUnderflow)
            } else {
                cpu.stack[cpu.stack_pointer - 2] *= cpu.stack[cpu.stack_pointer - 1];
                cpu.stack_pointer -= 1;
                InstructionExecutionResult::SuccessWithoutOutput(cpu)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    /// Insert the value on the top of the stack
    ///
    /// Expected InstructionExecutionResult:
    /// If the stack is full => Error(StackOverflow)
    /// otherwise => SuccessWithoutOutput
    Push(u64),
    /// Pop the value that currently resides on top of the stack.
    ///
    /// Expected InstructionExecutionResult:
    /// If the stack is empty => Error(StackUnderflow)
    /// otherwise => SuccessWithOutput(popped_value)
    Pop,
    /// Take two numbers of the top of the stack, add them, put the result on the stack.
    ///
    /// Expected InstructionExecutionResult:
    /// If the stack holds less than two values => Error(StackUnderflow)
    /// otherwise => SuccessWithoutOutput
    Add,
    /// Take two numbers of the top of the stack, multiply them, put the result on the stack.
    ///
    /// Expected InstructionExecutionResult:
    /// If the stack holds less than two values => Error(StackUnderflow)
    /// otherwise => SuccessWithoutOutput
    Multiply,
}

#[derive(Debug)]
enum InstructionExecutionResult {
    SuccessWithoutOutput(Cpu),
    SuccessWithOutput(u64, Cpu),
    Error(InstructionExecutionError),
}

#[derive(Debug)]
enum InstructionExecutionError {
    StackOverflow,
    StackUnderflow,
}

fn main() {
    // Push--Pop
    let mut cpu = new_cpu();
    cpu = match execute_instruction(cpu, Instruction::Push(42)) {
        InstructionExecutionResult::SuccessWithoutOutput(cpu) => cpu,
        _ => panic!(),
    };
    match execute_instruction(cpu, Instruction::Pop) {
        InstructionExecutionResult::SuccessWithOutput(output, _) => assert_eq!(output, 42),
        _ => panic!(),
    };

    // Bigger Push--Pop
    let mut cpu = new_cpu();
    cpu = match execute_instruction(cpu, Instruction::Push(42)) {
        InstructionExecutionResult::SuccessWithoutOutput(cpu) => cpu,
        _ => panic!(),
    };
    cpu = match execute_instruction(cpu, Instruction::Push(1)) {
        InstructionExecutionResult::SuccessWithoutOutput(cpu) => cpu,
        _ => panic!(),
    };
    cpu = match execute_instruction(cpu, Instruction::Push(101)) {
        InstructionExecutionResult::SuccessWithoutOutput(cpu) => cpu,
        _ => panic!(),
    };
    cpu = match execute_instruction(cpu, Instruction::Pop) {
        InstructionExecutionResult::SuccessWithOutput(output, cpu) => {
            assert_eq!(output, 101);
            cpu
        }
        _ => panic!(),
    };
    cpu = match execute_instruction(cpu, Instruction::Pop) {
        InstructionExecutionResult::SuccessWithOutput(output, cpu) => {
            assert_eq!(output, 1);
            cpu
        }
        _ => panic!(),
    };
    match execute_instruction(cpu, Instruction::Pop) {
        InstructionExecutionResult::SuccessWithOutput(output, _) => assert_eq!(output, 42),
        _ => panic!(),
    };

    // Minimal addition example
    let mut cpu = new_cpu();
    cpu = match execute_instruction(cpu, Instruction::Push(10)) {
        InstructionExecutionResult::SuccessWithoutOutput(cpu) => cpu,
        _ => panic!(),
    };
    cpu = match execute_instruction(cpu, Instruction::Push(15)) {
        InstructionExecutionResult::SuccessWithoutOutput(cpu) => cpu,
        _ => panic!(),
    };
    cpu = match execute_instruction(cpu, Instruction::Add) {
        InstructionExecutionResult::SuccessWithoutOutput(cpu) => cpu,
        _ => panic!(),
    };
    match execute_instruction(cpu, Instruction::Pop) {
        InstructionExecutionResult::SuccessWithOutput(output, _) => assert_eq!(output, 25),
        _ => panic!(),
    };

    // Minimal multiplication example
    let mut cpu = new_cpu();
    cpu = match execute_instruction(cpu, Instruction::Push(10)) {
        InstructionExecutionResult::SuccessWithoutOutput(cpu) => cpu,
        _ => panic!(),
    };
    cpu = match execute_instruction(cpu, Instruction::Push(15)) {
        InstructionExecutionResult::SuccessWithoutOutput(cpu) => cpu,
        _ => panic!(),
    };
    cpu = match execute_instruction(cpu, Instruction::Multiply) {
        InstructionExecutionResult::SuccessWithoutOutput(cpu) => cpu,
        _ => panic!(),
    };
    match execute_instruction(cpu, Instruction::Pop) {
        InstructionExecutionResult::SuccessWithOutput(output, _) => assert_eq!(output, 150),
        _ => panic!(),
    };

    // StackOverflow
    let mut cpu = new_cpu();
    cpu = match execute_instruction(cpu, Instruction::Push(1)) {
        InstructionExecutionResult::SuccessWithoutOutput(cpu) => cpu,
        _ => panic!(),
    };
    cpu = match execute_instruction(cpu, Instruction::Push(2)) {
        InstructionExecutionResult::SuccessWithoutOutput(cpu) => cpu,
        _ => panic!(),
    };
    cpu = match execute_instruction(cpu, Instruction::Push(3)) {
        InstructionExecutionResult::SuccessWithoutOutput(cpu) => cpu,
        _ => panic!(),
    };
    cpu = match execute_instruction(cpu, Instruction::Push(4)) {
        InstructionExecutionResult::SuccessWithoutOutput(cpu) => cpu,
        _ => panic!(),
    };
    cpu = match execute_instruction(cpu, Instruction::Push(5)) {
        InstructionExecutionResult::SuccessWithoutOutput(cpu) => cpu,
        _ => panic!(),
    };
    match execute_instruction(cpu, Instruction::Push(6)) {
        InstructionExecutionResult::Error(InstructionExecutionError::StackOverflow) => {}
        _ => panic!(),
    };

    // StackUnderflow Pop
    let cpu = new_cpu();
    match execute_instruction(cpu, Instruction::Pop) {
        InstructionExecutionResult::Error(InstructionExecutionError::StackUnderflow) => {}
        _ => panic!(),
    };

    // Add with empty stack StackUnderflow
    let cpu = new_cpu();
    match execute_instruction(cpu, Instruction::Add) {
        InstructionExecutionResult::Error(InstructionExecutionError::StackUnderflow) => {}
        _ => panic!(),
    };

    // Multiply with only one value on the stack StackUnderflow
    let mut cpu = new_cpu();
    cpu = match execute_instruction(cpu, Instruction::Push(5)) {
        InstructionExecutionResult::SuccessWithoutOutput(cpu) => cpu,
        _ => panic!(),
    };
    match execute_instruction(cpu, Instruction::Add) {
        InstructionExecutionResult::Error(InstructionExecutionError::StackUnderflow) => {}
        _ => panic!(),
    };

    println!("Your code seems to work well!");
}
