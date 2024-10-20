use task_fs::Status;

mod task_fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut todo = Todo::new();

    match args.len() {
        2 => {
            match args[1].as_str() {
                "list"  => todo.ls(),
                _       => help(),
            }
        },
        3 => {
            let opt = args[2].to_string();
            // Handle errors in mv(), ls().
            match args[1].as_str() {
                "add"               => todo.add(opt),
                "delete"            => todo.mv(opt, "gone".to_string()),
                "mark-done"         => todo.mv(opt, "done".to_string()),
                "mark-in-progress"  => todo.mv(opt, "_ing".to_string()),
                "list"              => todo.ls(opt),
                _                   => help()
            }
        },
        4 => {
            let (tid, detail) = (args[3].to_string(), args[4].to_string());
            todo.mv(tid, detail);
        },
        _ => help(),
    }
}

fn help() {
    println!("Usage: task_cli <action> [option]");
}
