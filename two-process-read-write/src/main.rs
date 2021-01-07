use libc;
use nix::unistd::{fork, ForkResult};
use std::ffi::CString;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;

unsafe fn add_num(file_name: &str, lock: *mut libc::sem_t) {
    // 打开文件
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)
        .unwrap();

    for _ in 0..2000 {
        // 如果为lock第值为0就等待, 不为0减去1
        libc::sem_wait(lock);

        // 数据buffer
        let mut buffer = String::new();

        // 设置文件指针到0号位置
        file.seek(SeekFrom::Start(0)).unwrap();

        // 从文件读取到buffer里面
        file.read_to_string(&mut buffer).unwrap();

        // buffer转成数字
        let mut size = buffer.parse::<i32>().unwrap();

        // 累加
        size += 1;

        // 数字转成string
        let to_data = format!("{:04}", size);

        // 偏移到文件指针0号位置
        file.seek(SeekFrom::Start(0)).unwrap();

        // 写到文件里面
        file.write(&to_data[..].as_bytes()).unwrap();

        // lock的值+1
        libc::sem_post(lock);
    }
}

fn main() {
    let file_name = "./test.dat";

    let mut f = File::create(file_name).unwrap();
    f.write(b"0000").unwrap();

    // 创建有名信号量字符串
    let path = CString::new("asr.lock.dat").expect("CString::new failed");

    unsafe {
        libc::sem_unlink(path.as_ptr());
        let lock = libc::sem_open(path.as_ptr(), libc::O_CREAT | libc::O_EXCL, 0644, 1);

        // fork父子进程
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                println!(
                    "Continuing execution in parent process, new child has pid: {}",
                    child
                );
                add_num(file_name, lock);
            }

            Ok(ForkResult::Child) => add_num(file_name, lock),
            Err(_) => println!("Fork failed"),
        }
    }
}
