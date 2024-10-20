use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    tid: u32,
    status: String,
    detail: String,
}

struct Todo {
    data: Vec<Task>,
}

mod lib {
    impl Todo {
        fn new() -> Self {
            Todo {
                data: Todo::load().unwrap_or_default(),
            }
        } 

        fn load() -> Result<Vec<Task>, Box<dyn Error>> {
            let path = Path::new("log.json");
            if path.exists() {
                let file = File::open(path)?;
                let reader = BufReader::new(file);
                let data: Vec<Task> = serde_json::from_reader(reader)?;
                Ok(data)
            } else {
                Ok(Vec::new())
            }
        }

        fn save(&self) -> Result<(), Box<dyn Error>> {
             let file = File::create("log.json")?;
             let writer = BufWriter::new(file);
             serde_json::to_writer_pretty(writer, &self.data)?;
             Ok(())
        }
    
        fn ls(&self, opt: String) {
            if is_status(&opt) {
                if let Some(i) = &self.data.iter().find(|i| i.status == opt) {
                    println!("{} {}", i.tid, i.detail);
                }
            } else if opt == "all" {
                if let Some(i) = &self.data.iter().find(|i| i.status != "gone") {
                println!("{} {} {}", i.tid, i.status, i.detail);
            } 
            }else {
                println!("Invalid status provided.");
            }
        }

        fn add(&mut self, detail: String) {
            self.data.push(Task {
                tid: self.data.len() as u32,
                status: "todo".to_string(),
                detail,
            });
            self.save().unwrap();
        }

        fn mv(&mut self, tid_str: String, opt: String) {
            if let Ok(tid) = tid_str.parse::<u32>() {
                if let Some(item) = self.data.iter_mut().find(|item| item.tid == tid) {
                    if is_status(&opt) {
                        item.status = opt;
                    } else {
                        item.detail = opt;
                    }
                    self.save().unwrap();
                } else {
                    println!("Task with ID {} not found", tid);
                }
            } else {
                println!("Invalid task_id provided.");
            }
        }
    }
}

fn is_status(opt: &str) -> bool {
    matches!(opt, "todo" | "_ing" | "done" | "gone")
}
