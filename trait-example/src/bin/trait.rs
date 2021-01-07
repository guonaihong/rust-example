// work trait
trait Person {
    fn work(&self);
}

// 程序员
struct Programmer {}

// 老师
struct Teacher {}

// 给老师实现work方法
impl Person for Teacher {
    fn work(&self) {
        println!("Teacher:每年有寒暑假");
    }
}

// 给程序员实现work方法
impl Person for Programmer {
    fn work(&self) {
        println!("Programmer:每周有996");
    }
}

// p是一个实现Person函数
fn do_work(p: impl Person) {
    p.work()
}

fn main() {
    do_work(Programmer {});
    do_work(Teacher {});
}
