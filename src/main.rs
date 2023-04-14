mod args;
mod log_path;
mod parser;

fn main() {
    // Demo
    let paths = args::input();
    let logs_pool = log_path::find_logs(paths);
    

    /*
    let keyword = "Elapsed time for this session";
    let lines = parser::find_keyword(&path, &keyword).unwrap();
    for line in lines {
        println!("{}", line);
    }
    */

    //let user_observer = parser::UserInfo::new("siteid","user");
    let mut subject = parser::LogSubject::new();
    let user_observer = parser::UserInfo {};
    let running_observer = parser::RunningInfo {};
    subject.attach(Box::new(user_observer));
    subject.attach(Box::new(running_observer));
    for item in logs_pool {
        subject.process_log_buf(&item);
    }
    //subject.process_log_buf(&path);
    //let files = log_path::find_logs(path);
    //println!("{:?}", files);
}
