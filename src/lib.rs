struct Time {}

enum Status {
    todo,
    _ing,
    done,
    gone,
}

struct Task {
    tid: u32,
    detail: (Status, String),
    createAt: Time,
    updateAt: Time,
}

struct Todo {
    arr: vec!(Task)
}

impl Todo {
    fn load() {

    }

    fn save() {
        
    }
    
    fn ls() {
        vec = load();
        for line in vec {
            println!();
        }
    }

    fn add(detail) {
        vec = load();
        vec.append(Task(detail));
        save(vec);
    }

    fn mv(tid, detail) {
        use Status;

        vec = load();
        match type detail {
            Status => , vec.locate(tid).detail.0 = detail
            _ => vec.locate(tid).detail.1 = detail
        }
        vec.locate(tid).status = detail;
        save(vec);
    }
}
