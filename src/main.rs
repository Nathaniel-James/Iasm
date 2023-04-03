use std::collections::HashMap;
use std::time::Duration;
use std::thread::sleep;

#[derive(Debug)]
struct ProgramVariable {
    program: Vec<Vec<String>>,
    stack: Vec<isize>,
    var_tracker: HashMap<String, isize>,
    pnt_tracker: HashMap<String, usize>,
    current_line: usize,
    error_tracker: u8,
}

impl ProgramVariable {
    // Arithmetic 

    fn add(&mut self, number: isize) {
        let stack_option = self.stack.last();   // Get stack number

        match stack_option {                                    // Error handling
            None => {self.error_tracker = 2;},                  // 2: Stack Empty Error
            Some(x) => {
                let result: isize = x + number;                 // Add (stack + number) and push to stack
                self.stack.push(result);}
        };
    
        self.error_tracker = 0;
    }

    fn sub(&mut self, number: isize) {
        let stack_option = self.stack.last();   // Get stack number

        match stack_option {                                    // Error handling
            None => {self.error_tracker = 2;},                  // 2: Stack Empty Error
            Some(x) => {
                let result: isize = x - number;                 // Add (stack + number) and push to stack
                self.stack.push(result);}
        };
    
        self.error_tracker = 0;
    }

    fn mul(&mut self, number: isize) {

    }

    fn div(&mut self, number: isize) {

    }



    // Masks
    fn and() {}
    fn or() {}
    fn xor() {}
    fn not() {}



    // Variable management

    fn assign(&mut self, name: &String) {
        let value = self.stack.last();          // Get stack number

        match value {                                           // Error handling
            None => {self.error_tracker = 2;},                  // 2: Stack Empty Error
            Some(x) => {self.var_tracker.insert(name.to_string(), *x);}  // Assign name (key) to stack.last (value)
        };
    
        self.error_tracker = 0;
    }

    fn get(&mut self, name:&String) {
        let var = self.var_tracker.get(name);

        match var {                                             // Error handling
            None => {self.error_tracker = 1},                   // 1: Name Error
            Some(x) => {self.stack.push(*x);}
        }

        self.error_tracker = 0;
    }



    // User Input and Console Output

    fn input(&mut self) {
        // Reading input
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("stdin error");

        // Processing input
        let trimmed = line.trim();
        let isize_line = trimmed.parse::<isize>();

        match isize_line {
            Ok(x) => {self.stack.push(x);},
            Err(_) => {                                     // Err pushes each char of string (line) to the stack 
                let isize_list: Vec<isize> = trimmed.split_whitespace().map(|s| s.parse().unwrap()).collect();  // TODO: potential issue? (thanks stackoverflow)
                
                for char in isize_list {
                    self.stack.push(char);
                }
            }
        }

        self.error_tracker = 0;
    }

    fn output_number(&mut self) {
        let stack_option = self.stack.last();

        match stack_option {                                // Error handling
            None => {self.error_tracker = 1;},
            Some(x) => {
                let content = x.to_string();
                println!("{}", content);
            }
        };
    
        self.error_tracker = 0;
    }

    fn output_ascii(&mut self) {
        let stack_option = self.stack.last();

        match stack_option {
            None => {self.error_tracker = 1;},
            Some(x) => {
                let content = char::from(*x as u8);
                println!("{}", content); 
            }
        }
    }

    fn output_unicode(&mut self) {
        let stack_option = self.stack.last();

        match stack_option {
            None => {self.error_tracker = 1;},
            Some(x) => {
                let content = std::char::from_u32(*x as u32);
                
                match content {
                    None => {self.error_tracker = 1;},
                    Some(y) => {println!("{}", y);} 
                }
            }
        }
    }




    // Stack manipulation

    fn push(&mut self, value: isize) {                                                               // Stack manipuation
        self.stack.push(value);
        self.error_tracker = 0;
    }
    
    fn pop(&mut self) {
        let element = self.stack.pop();
    
        match element {
            None => {self.error_tracker = 2;},
            Some(_) => {self.error_tracker = 0;}
        };
    }



    // Jump points

    fn create_point(&mut self, name: String) {
        self.pnt_tracker.insert(name, self.current_line);
        self.error_tracker = 0;
    }
    
    fn jump_point(&mut self, name: String) {
        let line_no = self.pnt_tracker.get(&name);
    
        match line_no {
            None => {self.error_tracker = 1;},
            Some(x) => {
                self.current_line = *x;
                self.error_tracker = 0;
            }
        };
    }
    
    fn jump_line(&mut self, number: usize) {
        self.current_line = number;
        self.error_tracker = 0;
    }



    // Conditional checks

    fn check_equal(&mut self, number: isize) {
        let stack_option = self.stack.last();

        match stack_option {
            None => {self.error_tracker = 1;},
            Some(x) => {
                if number == *x {
                    self.stack.push(1);
                } else {
                    self.stack.push(0);
                }
            }
        }
    }

    fn check_less(&mut self, number: isize) {
        let stack_option = self.stack.last();

        match stack_option {
            None => {self.error_tracker = 1;},
            Some(x) => {
                if number < *x {
                    self.stack.push(1);
                } else {
                    self.stack.push(0);
                }
            }
        }
    }

    fn check_greater(&mut self, number: isize) {
        let stack_option = self.stack.last();

        match stack_option {
            None => {self.error_tracker = 1;},
            Some(x) => {
                if number > *x {
                    self.stack.push(1);
                } else {
                    self.stack.push(0);
                }
            }
        }
    }



    // Branches

    fn jump_zero(&mut self, name: String) {
        let stack_option = self.stack.last();

        match stack_option {
            None => {self.error_tracker = 1;},
            Some(x) => {
                if *x == 0 {
                    let point = self.pnt_tracker.get(&name);

                    match point {
                        None => {self.error_tracker = 1;},
                        Some(x) => {self.current_line = *x;}
                    }
                }
            }
        }
    }

    fn jump_even(&mut self, name: String) {
        let stack_option = self.stack.last();

        match stack_option {
            None => {self.error_tracker = 1;},
            Some(x) => {
                if (*x%2) == 0 {
                    let point = self.pnt_tracker.get(&name);

                    match point {
                        None => {self.error_tracker = 1;},
                        Some(x) => {self.current_line = *x;}
                    }
                }
            }
        }
    }
}


// File I/O
fn open() -> Vec<Vec<String>> {
    return vec![
        vec![String::from("PUS"), String::from("0")],
        vec![String::from("CHL"), String::from("1")],
        vec![String::from("OPN")]
    ];
}

fn main() {
    // Program instance
    let mut instance = ProgramVariable {
        program: open(),
        stack: vec![],
        var_tracker: HashMap::new(),
        pnt_tracker: HashMap::new(),
        current_line: 0,
        error_tracker: 0,
    };

    // Debug variables
    let debug_print_instance: bool = true;
    let debug_pause: bool = true;
    let pause = Duration::from_secs(1);

    // FDE cycle
    while instance.error_tracker == 0 {
        if debug_pause {sleep(pause);}
        if debug_print_instance {println!("\n{:?}\n", instance);}
        
        let current_command: &Vec<String> = &instance.program[instance.current_line];   // Fetch current command Vec<String>

        match &current_command[0] as &str {                                             // Execute (match String to command)
            "ADD" => {                                                                  // Arithmetic operations
                let num_arg = current_command[1].parse();

                match num_arg {
                    Err(_) => {} 
                    Ok(x) => {instance.add(x);}
                }
            },
            "SUB" => {
                let num_arg = current_command[1].parse();

                match num_arg {
                    Err(_) => {} 
                    Ok(x) => {instance.sub(x);}
                }
            },
            "ASN" => {                                                                  // Variable operations
                let str_arg = current_command[1].to_string();
                instance.assign(&str_arg);
            },
            "GET" => {
                let str_arg = current_command[1].to_string();
                instance.get(&str_arg);
            },
            "INP" => {                                                                  // User I/O
                instance.input();
            },
            "OPN" => {
                instance.output_number();
            },
            "OPA" => {
                instance.output_ascii();
            },
            "OPU" => {
                instance.output_unicode();
            },
            "PUS" => {                                                                  // Stack operations
                let num_arg = current_command[1].parse();
                
                match num_arg {
                    Err(_) => {}
                    Ok(x) => {instance.push(x);}
                }
            },
            "POP" => {
                instance.pop();
            },
            "PNT" => {                                                                  // Jump/current_line manipulation operations
                let str_arg = current_command[1].to_string();
                instance.create_point(str_arg);
            },
            "JMP" => {
                let str_arg = current_command[1].to_string();
                instance.jump_point(str_arg);
            },
            "JML" => {
                let num_arg = current_command[1].parse();

                match num_arg {
                    Err(_) => {}
                    Ok(x) => {instance.jump_line(x);}
                }
            },
            "CHE" => {                                                                  // Conditional checks (equal, less than, greater than)
                let num_arg = current_command[1].parse();

                match num_arg {
                    Err(_) => {}
                    Ok(x) => {instance.check_equal(x);}
                }
            },
            "CHL" => {
                let num_arg = current_command[1].parse();

                match num_arg {
                    Err(_) => {}
                    Ok(x) => {instance.check_less(x);}
                }
            },
            "CHG" => {
                let num_arg = current_command[1].parse();

                match num_arg {
                    Err(_) => {}
                    Ok(x) => {instance.check_greater(x);}
                }
            },
            "JMZ" => {                                                                  // Branches (jump if x)
                let str_arg = current_command[1].to_string();
                instance.jump_zero(str_arg);
            },
            "JME" => {
                let str_arg = current_command[1].to_string();
                instance.jump_even(str_arg);
            }, 
            _ => {}
        }

        instance.current_line += 1;

        if instance.current_line == instance.program.len() {instance.error_tracker = 3;}
    }
    println!("\n-------------------");
    println!("Code exited with: {}", instance.error_tracker);
    println!("Stack:\n{:?}", instance.stack);
    println!("Variables:\n{:?}", instance.var_tracker);
    println!("Jump points:\n{:?}", instance.pnt_tracker);
}