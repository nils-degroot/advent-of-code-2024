#[derive(Debug)]
struct Vm {
    program: Vec<i64>,
    instruction_counter: i64,
    a: i64,
    b: i64,
    c: i64,
    output: Vec<i64>,
}

impl Vm {
    fn run(&mut self) -> Vec<i64> {
        while let (Some(opcode), Some(operand)) = (
            self.program.get(self.instruction_counter as usize),
            self.program.get(self.instruction_counter as usize + 1),
        ) {
            match opcode {
                0 => self.adv(*operand),
                1 => self.bxl(*operand),
                2 => self.bst(*operand),
                3 => self.jnz(*operand),
                4 => self.bxc(*operand),
                5 => self.out(*operand),
                6 => self.bdv(*operand),
                7 => self.cdv(*operand),
                _ => panic!("Invalid opcode"),
            }
        }

        self.output.clone()
    }

    fn combo_operand(&self, operand: i64) -> i64 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid operand provided"),
        }
    }

    fn bump_instruction_counter(&mut self) {
        self.instruction_counter += 2;
    }

    fn adv(&mut self, operand: i64) {
        let operand = self.combo_operand(operand);
        self.a /= 2i64.pow(operand as u32);
        self.bump_instruction_counter();
    }

    fn bxl(&mut self, operand: i64) {
        self.b ^= operand;
        self.bump_instruction_counter();
    }

    fn bst(&mut self, operand: i64) {
        self.b = self.combo_operand(operand) % 8;
        self.bump_instruction_counter();
    }

    fn jnz(&mut self, operand: i64) {
        if self.a == 0 {
            self.bump_instruction_counter();
            return;
        }

        self.instruction_counter = self.combo_operand(operand);
    }

    fn bxc(&mut self, _operand: i64) {
        self.b ^= self.c;
        self.bump_instruction_counter();
    }

    fn out(&mut self, operand: i64) {
        self.output.push(self.combo_operand(operand) % 8);
        self.bump_instruction_counter();
    }

    fn bdv(&mut self, operand: i64) {
        let operand = self.combo_operand(operand);
        self.b = self.a / 2i64.pow(operand as u32);
        self.bump_instruction_counter();
    }

    fn cdv(&mut self, operand: i64) {
        let operand = self.combo_operand(operand);
        self.c = self.a / 2i64.pow(operand as u32);
        self.bump_instruction_counter();
    }
}

pub fn part1(input: String) -> impl ToString {
    let mut parts = input.trim_end().split("\n\n");

    let mut registers = parts
        .next()
        .expect("Could not find registers")
        .lines()
        .map(|line| {
            line.split(": ")
                .nth(1)
                .expect("Could not find register value")
                .parse::<i64>()
                .expect("Could not parse register")
        });

    let program = parts
        .next()
        .expect("Could not find program")
        .split(": ")
        .skip(1)
        .flat_map(|part| part.split(","))
        .map(|part| part.parse::<i64>().expect("Could not parse program"))
        .collect::<Vec<_>>();

    let mut vm = Vm {
        a: registers.next().expect("Could not find register"),
        b: registers.next().expect("Could not find register"),
        c: registers.next().expect("Could not find register"),
        instruction_counter: 0,
        output: vec![],
        program,
    };

    let output = vm
        .run()
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>();

    output.join(",")
}

pub fn part2(input: String) -> impl ToString {
    let mut parts = input.trim_end().split("\n\n");

    let program = parts
        .nth(1)
        .expect("Could not find program")
        .split(": ")
        .skip(1)
        .flat_map(|part| part.split(","))
        .map(|part| part.parse::<i64>().expect("Could not parse program"))
        .collect::<Vec<_>>();

    let mut a = 0;

    for n in 0..program.len() {
        let target = &program[program.len() - (n + 1)..];

        let mut new_a = a << 3;

        loop {
            let mut vm = Vm {
                a: new_a,
                b: 0,
                c: 0,
                program: program.clone(),
                instruction_counter: 0,
                output: vec![],
            };

            if vm.run() == target {
                a = new_a;
                break;
            }

            new_a += 1;
        }
    }

    a
}
