// work trait
trait Person {
    fn work(&self);
}

// 程序员
struct Programmer {
    age: i32,
}

// 老师
struct Teacher {
    age: i32,
}

// 给老师实现work方法
impl Person for Teacher {
    fn work(&self) {
        println!("{}岁之后，当Teacher:每年有寒暑假", self.age);
    }
}

// 给程序员实现work方法
impl Person for Programmer {
    fn work(&self) {
        println!("{}岁之前，做Programmer:每周有996", self.age);
    }
}

//   error[E0308]: `if` and `else` have incompatible types
//     --> src/bin/dyn-trait.rs:36:9
//      |
//   33 | /     if age <= 35 {
//   34 | |         Programmer {}
//      | |         ------------- expected because of this
//   35 | |     } else {
//   36 | |         Teacher {}
//      | |         ^^^^^^^^^^ expected struct `Programmer`, found struct `Teacher`
//   37 | |     }
//      | |_____- `if` and `else` have incompatible types
//      |
//   help: you could change the return type to be a boxed trait object
//      |
//   32 | fn select_person(age: i32) -> Box<dyn Person> {
//      |                               ^^^^^^^       ^
//   help: if you change the return type to expect trait objects, box the returned expressions
//      |
//   34 |         Box::new(Programmer {})
//   35 |     } else {
//   36 |         Box::new(Teacher {})
//      |
//
//   error: aborting due to previous error

// 根据条件选择做什么职业
//   fn select_person(age: i32) -> impl Person {
//       if age <= 35 {
//           Programmer {}
//       } else {
//           Teacher {}
//       }
//   }

fn dyn_do_work(p: Box<dyn Person>) {
    p.work();
}

fn select_person(age: i32) -> Box<dyn Person> {
    if age <= 35 {
        Box::new(Programmer { age: age })
    } else {
        Box::new(Teacher { age: age })
    }
}

fn main() {
    dyn_do_work(select_person(30));
    dyn_do_work(select_person(36));
}
