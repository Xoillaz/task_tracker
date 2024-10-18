use std::env;
use lib::Todo;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            match args[1] {
                "list"  => Todo.ls("all"),
                _       => help(),
            }
        },
        3 => {
            let opt = &args[2];
            match args[1] {
                "add"               => Todo.add(opt),
                "delete"            => Todo.mk(opt, "gone"),
                "mark-done"         => Todo.mk(opt, "done"),
                "mark-in-progress"  => Todo.mk(opt, "_ing"),
                "list"              => Todo.ls(opt),
                _                   => help()
            }
        },
        4 => {
            let tid, detail = &args[2], &args[3];
            match args[1] {
                "update"    => Todo.mv(tid, detail),
                _           => help(),
            }
        },
        _ => help(),
    }

fn help() {
    println!()
}
