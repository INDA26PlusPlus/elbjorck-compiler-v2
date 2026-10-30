use std::{char, fs::read_to_string, ops::Add, range};

fn main() {
    run_code("src/code.txt");
}

fn run_code(file_name: &str) {
    let mut memory: Vec<usize> = Vec::new();
    let code: Vec<String> = read_lines(file_name);
    let tokens: Vec<(String, String)> = tokenize_lines(code);
    let tree = create_syntax_tree(tokens.clone());

    print_tokens(tokens.clone());
    print_syntax_tree(create_syntax_tree(tokens));

    run_block(&tree, &mut memory, "main");
    print!("memory: {{");
    for value in memory {
        print!(" {}", value);
    }
    println!(" }}");
}

fn run_block(tree: &Vec<Vec<(String, String)>>, memory: &mut Vec<usize>, identifier: &str) {
    //println!("Running block {}", identifier);
    for block in tree {
        let mut line: Vec<(String, String)> = Vec::new();
        if block.get(0).unwrap().1 == identifier {
            for i in 0..block.len() {
                let token = block.get(i).unwrap();

                if i < 1 {
                    continue;
                }
                if token.1 != ";" {
                    //println!("Adding token to line {}", token.1);
                    line.push(token.clone());
                } else {
                    //println!("running line with tokens:");
                    print_tokens(line.clone());
                    run_line(tree, &line, memory);
                    line.clear();
                }
            }
        }
    }
}

fn run_line(
    tree: &Vec<Vec<(String, String)>>,
    line: &Vec<(String, String)>,
    memory: &mut Vec<usize>,
) {
    println!("Running line");
    match line.get(0).unwrap().1.as_str() {
        "add" => {
            add(line, memory);
        }
        "push" => {
            push(line, memory);
        }
        "pop" => {
            pop(line, memory);
        }
        "run" => {
            run(tree, line, memory);
        }
        "if" => {
            if_command(tree, line, memory);
        }
        "look" => {
            look(line, memory);
        }

        _ => {}
    }
}

fn if_command(
    tree: &Vec<Vec<(String, String)>>,
    line: &Vec<(String, String)>,
    memory: &mut Vec<usize>,
) {
    let mut identifier: Vec<(String, String)> = Vec::new();
    let mut values: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < line.len() {
        let token = line.get(i).unwrap();
        if token.0 == "value" {
            values.push(token.1.parse::<usize>().unwrap());
        }
        if token.0 == "identifier" {
            identifier.push(token.clone());
        }
        if token.1 == "pop" || token.1 == "look" {
            let mut p_tokens: Vec<(String, String)> = Vec::new();
            for j in i..line.len() {
                let p_token: (String, String) = line.get(j).unwrap().clone();
                if p_token.1 != ")" {
                    p_tokens.push(p_token);
                } else {
                    break;
                }
            }
        }
        i += 1;
    }
    for i in 0..values.len() - 1 {
        if values.get(i).unwrap() == values.get(i + 1).unwrap() {
            continue;
        } else {
            return;
        }
    }
    run(tree, &identifier, memory);
}

fn run(tree: &Vec<Vec<(String, String)>>, tokens: &Vec<(String, String)>, memory: &mut Vec<usize>) {
    for token in tokens {
        if token.0 == "identifier" {
            run_block(tree, memory, token.1.as_str());
            return;
        }
    }
}

fn add(tokens: &Vec<(String, String)>, memory: &mut Vec<usize>) {
    let mut values: [usize; 2] = [0, 1];
    let mut index: usize = 0;
    let mut i = 0;
    while i < tokens.len() {
        let token = tokens.get(i).unwrap();
        if token.0 == "value" {
            values[index] = token.1.parse::<usize>().unwrap();
            index += 1;
        }
        if token.1 == "pop" || token.1 == "look" {
            let mut p_tokens: Vec<(String, String)> = Vec::new();
            for j in i..tokens.len() {
                let p_token = tokens.get(j).unwrap();
                p_tokens.push(p_token.clone());
                i += 1;
                if p_token.1 == ")" {
                    break;
                }
            }
            if token.1 == "pop" {
                let popped_value = pop(&p_tokens, memory);
                values[index] = popped_value;
            } else {
                let looked_value = look(&p_tokens, memory);
                values[index] = looked_value;
            }
            index += 1;
        }
        i += 1;
    }

    println!("Adding values at indexes {} {}", values[0], values[1]);

    println!(
        "Sum {}",
        memory.get(memory.len() - 1 - values[0]).unwrap().clone() + memory.get(memory.len() - 1 - values[1]).unwrap().clone()
    );
    memory.push(memory.get(memory.len() - 1 - values[0]).unwrap().clone() + memory.get(memory.len() - 1 - values[1]).unwrap().clone());
}

fn push(tokens: &Vec<(String, String)>, memory: &mut Vec<usize>) {
    //println!("running push command");
    for i in 0..tokens.len() {
        let token = tokens.get(i).unwrap();
        if token.0 == "value" {
            println!("pushing token to memory: {}", token.1);
            memory.push(token.1.parse::<usize>().unwrap());
            return;
        }
        if token.1 == "pop" || token.1 == "look" {
            let mut p_tokens: Vec<(String, String)> = Vec::new();
            for j in i..tokens.len() {
                let p_token = tokens.get(j).unwrap();
                p_tokens.push(p_token.clone());
                if p_token.1 == ")" {
                    break;
                }
            }
            if token.1 == "pop" {
                let popped_val = pop(&p_tokens, memory);
                memory.push(popped_val);
            }
        }
    }
}

fn pop(tokens: &Vec<(String, String)>, memory: &mut Vec<usize>) -> usize {
    let return_value: usize;
    for token in tokens {
        if token.0 == "value" {
            return_value = memory
                .get(memory.len() - 1 - token.1.parse::<usize>().unwrap())
                .unwrap()
                .clone();
            memory.remove(memory.len() - 1 - token.1.parse::<usize>().unwrap());
            return return_value;
        }
    }
    return_value = memory.get(memory.len() - 1).unwrap().clone();
    memory.remove(memory.len() - 1);
    return return_value;
}

fn look(tokens: &Vec<(String, String)>, memory: &mut Vec<usize>) -> usize {
    println!("Running look");
    let return_value: usize;
    for token in tokens {
        if token.0 == "value" {
            //println!("looking at specified value {} which looks at index {} of memory {}", token.1, memory.len() - 1 - token.1.parse::<usize>().unwrap(), memory.len());
            return_value = memory
                .get(memory.len() - 1 - token.1.parse::<usize>().unwrap())
                .unwrap()
                .clone();
            return return_value;
        }
    }
    return_value = memory.get(memory.len() - 1).unwrap().clone();
    return return_value;
}

//Skapa abstract syntax tree

//Finns inga block i block

//Hitta alla block metod
//Returnar lista med i64or som har värdet av indexet av ett "Block" token

fn find_blocks(tokens: Vec<(String, String)>) -> Vec<i64> {
    let mut index: i64 = 0;
    let mut block_start_indexes: Vec<i64> = Vec::new();
    for token in tokens {
        if token.1 == "Block" {
            println!("Found block");
            block_start_indexes.push(index.clone());
        }

        index += 1;
    }
    return block_start_indexes;
}

//Skapa alla block metod
//Tar emot lista med alla tokens
//Tar emot lista med värden på vilket token som är det sissta i blocket
//Returnerar lista av listor av tokens
//Varje lista är ett block

fn create_block(tokens: Vec<(String, String)>, start_index: i64) -> Vec<(String, String)> {
    let mut block: Vec<(String, String)> = Vec::new();
    let mut index: i64 = 0;
    for token in tokens {
        if index <= start_index || token.1 == "{" {
            index += 1;
            continue;
        }

        if token.1 == "}" {
            break;
        }
        block.push(token);
        index += 1;
    }
    return block;
}

fn create_syntax_tree(tokens: Vec<(String, String)>) -> Vec<Vec<(String, String)>> {
    let block_start_indexes: Vec<i64> = find_blocks(tokens.clone());
    let mut blocks: Vec<Vec<(String, String)>> = Vec::new();
    for index in block_start_indexes {
        blocks.push(create_block(tokens.clone(), index));
    }
    return blocks;
}

fn print_syntax_tree(tree: Vec<Vec<(String, String)>>) {
    for block in tree {
        println!("--- new block ---");
        print_tokens(block);
    }
}

fn print_tokens(tokens: Vec<(String, String)>) {
    if tokens.len() == 0 {
        println!("Printing tokens {{    }}");
        return;
    }
    println!("Printing tokens {{");
    for token in tokens {
        println!("( {}  {} )", token.0, token.1);
    }
    println!("}}");
}

fn read_lines(file_name: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for line in read_to_string(file_name).unwrap().lines() {
        result.push(line.to_string());
    }
    return result;
}

fn tokenize_lines(code: Vec<String>) -> Vec<(String, String)> {
    let mut tokens: Vec<(String, String)> = Vec::new();
    for line in code {
        tokens.append(&mut tokenize_line(&line));
    }

    //print_tokens(tokens.clone());
    return tokens;
}

fn tokenize_line(line: &str) -> Vec<(String, String)> {
    let mut tokens: Vec<(String, String)> = Vec::new();
    let mut token: String = "".to_string();

    for (index, char) in line.char_indices() {
        match char {
            ' ' | '(' | ')' | '{' | '}' | ',' | ';' => {
                if token != "" {
                    tokens.push(identify_token_type(token.clone()));
                    token = "".to_string();
                }

                if char != ' ' {
                    tokens.push(identify_token_type(char.to_string()));
                }
            }
            _ => {
                token.push(char);
            }
        }
    }

    //print_tokens(tokens.clone());
    return tokens;
}

fn identify_token_type(token: String) -> (String, String) {
    let token_type: (String, String);

    //Value

    if token.parse::<i64>().is_ok() {
        token_type = ("value".to_string(), token);
        return token_type;
    }

    match token.as_str() {
        //command
        "add" => {
            token_type = ("command".to_string(), token);
        }
        "push" => {
            token_type = ("command".to_string(), token);
        }
        "pop" => {
            token_type = ("command".to_string(), token);
        }
        "run" => {
            token_type = ("command".to_string(), token);
        }
        "if" => {
            token_type = ("command".to_string(), token);
        }
        "look" => {
            token_type = ("command".to_string(), token);
        }

        //punctuation
        "(" => {
            token_type = ("punctuation".to_string(), token);
        }
        ")" => {
            token_type = ("punctuation".to_string(), token);
        }
        "," => {
            token_type = ("punctuation".to_string(), token);
        }
        "{" => {
            token_type = ("punctuation".to_string(), token);
        }
        "}" => {
            token_type = ("punctuation".to_string(), token);
        }
        ";" => {
            token_type = ("punctuation".to_string(), token);
        }

        //keyword
        "Block" => {
            token_type = ("keyword".to_string(), token);
        }

        //Exception
        _ => {
            token_type = ("identifier".to_string(), token);
        }
    }

    //println!("Creating new token: ({}, '{}')", token_type.0, token_type.1);
    return token_type;
}
