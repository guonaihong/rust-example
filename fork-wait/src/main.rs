use nix::sys::wait;
use nix::unistd::{fork, ForkResult};
use std::{thread, time};

fn proc_new() {
    match fork() {
        // 父进程
        Ok(ForkResult::Parent { child, .. }) => {
            println!(
                "Continuing execution in parent process, new child has pid: {}",
                child
            );
        }
        // 子进程
        Ok(ForkResult::Child) => {
            let sec = time::Duration::from_secs(3);
            thread::sleep(sec);
            println!("I'm a new child process");

            // 模拟coredump　或者段错误
            panic!("child process dead");
        }
        Err(err) => println!("Fork failed:{}", err),
    }
}

fn main() {
    proc_new();

    // 监控子进程状态
    loop {
        match wait::wait() {
            Ok(status) => {
                println!("Child exited with status {:?}", status);
                proc_new();
            }

            Err(err) => panic!("waitpid() failed: {}", err),
        }
    }
}
