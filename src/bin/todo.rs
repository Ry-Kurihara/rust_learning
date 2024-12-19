use std::fs;
use text_colorizer::*;

const TODO_FILE: &str = "todo.txt";

fn print_usage() {
    eprintln!("{} - run todo application", "todo_task".green());
    eprintln!("Usage:");
    eprintln!("  cargo run -- add <task>");
    eprintln!("  cargo run -- list");
    eprintln!("  cargo run -- remove <task_number>");
}

fn load_todo_list() -> Vec<String> {
    match fs::read_to_string(TODO_FILE) {
        Ok(contents) => contents.lines().map(|s| s.to_string()).collect(),
        Err(_) => Vec::new(), // ファイルが存在しない場合は空のベクタを返す
    }
}

fn save_todo_list(todo_list: &Vec<String>) {
    let contents = todo_list.join("\n");
    fs::write(TODO_FILE, contents).expect("TODOリストの保存に失敗しました");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let command = &args[1];
    let mut todo_list = load_todo_list(); // ファイルからロード

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                print_usage();
                std::process::exit(1);
            }
            todo_list.push(args[2].clone());
            save_todo_list(&todo_list); // ファイルに保存
            println!("タスクを追加しました: {}", args[2].green());
        }
        "list" => {
            if todo_list.is_empty() {
                println!("TODOリストは空です。");
            } else {
                println!("TODOリスト:");
                for (i, task) in todo_list.iter().enumerate() {
                    println!("{}. {}", i + 1, task.blue());
                }
            }
        }
        "remove" => {
            if args.len() < 3 {
                print_usage();
                std::process::exit(1);
            }
            let index_to_remove = args[2].parse::<usize>().unwrap_or(0);
            if index_to_remove > 0 && index_to_remove <= todo_list.len() {
                let removed_task = todo_list.remove(index_to_remove - 1);
                save_todo_list(&todo_list); // ファイルに保存
                println!("タスクを削除しました: {}", removed_task.red());
            } else {
                eprintln!("無効なタスク番号です。");
            }
        }
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}