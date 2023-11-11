use std::collections::{HashMap, HashSet};

// 学生结构体
#[derive(Debug)]
struct Student {
    id: u64,
    name: String,
    age: u32,
    class_id: u64,   //班级
    club_ids: HashSet<u64>,   //学生所参加的社团
    course_ids: HashSet<u64>, //学生所参加的课程
}

// 社团结构体
#[derive(Debug)]
struct Club {
    id: u64,
    name: String,
    members: HashSet<u64>,
}

// 班级结构体
#[derive(Debug)]
struct Class {
    id: u64,
    name: String,
    students: HashSet<u64>,
}

// 课程结构体
#[derive(Debug)]
struct Course {
    id: u64,
    name: String,
    enrolled_students: HashSet<u64>,
}

// 学生管理系统结构体
struct StudentManagementSystem {
    students: HashMap<u64, Student>,
    clubs: HashMap<u64, Club>,
    classes: HashMap<u64, Class>,
    courses: HashMap<u64, Course>,
}

impl StudentManagementSystem {
    // 创建学生
    fn create_student(&mut self, id: u64, name: String, age: u32, class_id: u64) {
        let student = Student {
            id,
            name,
            age,
            class_id,
            club_ids: HashSet::new(),
            course_ids: HashSet::new(),
        };
        self.students.insert(id, student);
    }

    // 创建社团
    fn create_club(&mut self, id: u64, name: String) {
        let club = Club {
            id,
            name,
            members: HashSet::new(),
        };
        self.clubs.insert(id, club);
    }

    // 创建班级
    fn create_class(&mut self, id: u64, name: String) {
        let class = Class {
            id,
            name,
            students: HashSet::new(),
        };
        self.classes.insert(id, class);
    }

    // 创建课程
    fn create_course(&mut self, id: u64, name: String) {
        let course = Course {
            id,
            name,
            enrolled_students: HashSet::new(),
        };
        self.courses.insert(id, course);
    }

    // 读取学生信息
    fn read_student(&self, student_id: u64) -> Option<&Student> {
        self.students.get(&student_id)
    }

    // 更新学生信息
    fn update_student(&mut self, student_id: u64, new_name: String, new_age: u32) {
        if let Some(student) = self.students.get_mut(&student_id) {
            student.name = new_name;
            student.age = new_age;
        }
    }

    // 将学生添加到课程中
    fn enroll_student_in_course(&mut self, student_id: u64, course_id: u64) {
        if let Some(student) = self.students.get_mut(&student_id) {
            if let Some(course) = self.courses.get_mut(&course_id) {
                student.course_ids.insert(course_id);
                course.enrolled_students.insert(student_id);
            }
        }
    }

    // 从课程中移除学生
    fn remove_student_from_course(&mut self, student_id: u64, course_id: u64) {
        if let Some(student) = self.students.get_mut(&student_id) {
            if let Some(course) = self.courses.get_mut(&course_id) {
                student.course_ids.remove(&course_id);
                course.enrolled_students.remove(&student_id);
            }
        }
    }

    // 删除学生
    fn delete_student(&mut self, student_id: u64) {
        self.students.remove(&student_id);
    }
}

fn main() {
    let mut system = StudentManagementSystem {
        students: HashMap::new(),
        clubs: HashMap::new(),
        classes: HashMap::new(),
        courses: HashMap::new(),
    };

    // 创建示例数据
    system.create_student(1, "Alice".to_string(), 20, 101);
    system.create_student(2, "Bob".to_string(), 21, 101);
    system.create_club(101, "Programming Club".to_string());
    system.create_class(201, "Computer Science".to_string());
    system.create_course(301, "Rust Programming".to_string());

    // 输出学生信息
    if let Some(student) = system.read_student(1) {
        println!("Student 1: {:?}", student);
    }

    // 更新学生信息
    system.update_student(1, "Alicia".to_string(), 21);

    // 输出更新后的学生信息
    if let Some(student) = system.read_student(1) {
        println!("Updated Student 1: {:?}", student);
    }

    // 删除学生
    system.delete_student(1);

    // 尝试输出已删除的学生信息
    if let Some(student) = system.read_student(1) {
        println!("Student 1 after deletion: {:?}", student);
    } else {
        println!("Student 1 not found (deleted).");
    }

    // 将学生2添加到课程301中
    system.enroll_student_in_course(2, 301);

    // 输出课程信息，检查学生是否成功添加
    if let Some(course) = system.courses.get(&301) {
        println!("Course 301: {:?}", course);
    }

    // 从课程301中移除学生1
    system.remove_student_from_course(2, 301);

    // 输出更新后的课程信息，检查学生是否成功移除
    if let Some(course) = system.courses.get(&301) {
        println!("Course 301 after removal: {:?}", course);
    }

}
