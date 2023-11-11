use std::collections::{HashMap, HashSet};

// 学生结构体
#[derive(Debug)]
struct Student {
    id: u64,
    name: String,
    age: u32,
    class_id: u64,
    course_enrollments: HashSet<CourseEnrollment>, // 多对多关系：学生和课程
    club_memberships: HashSet<ClubMembership>,     // 多对多关系：学生和社团
}

// 社团结构体
#[derive(Debug)]
struct Club {
    id: u64,
    name: String,
}

// 社团成员关系结构体
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct ClubMembership {
    student_id: u64,
    club_id: u64,
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
}

// 学生和课程关系结构体
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct CourseEnrollment {
    student_id: u64,
    course_id: u64,
}
// 学生管理系统结构体
struct StudentManagementSystem {
    students: HashMap<u64, Student>,
    clubs: HashMap<u64, Club>,
    classes: HashMap<u64, Class>,
    courses: HashMap<u64, Course>,
}

impl StudentManagementSystem {
    // 学生相关操作
    // 创建
    fn create_student(&mut self, id: u64, name: String, age: u32, class_id: u64) {
        let student = Student {
            id,
            name,
            age,
            class_id,
            course_enrollments: HashSet::new(),
            club_memberships: HashSet::new(),
        };
        self.students.insert(id, student);
    }

    // 更新
    fn update_student(&mut self, student_id: u64, new_name: String, new_age: u32) {
        if let Some(student) = self.students.get_mut(&student_id) {
            student.name = new_name;
            student.age = new_age;
        }
    }

    // 查看
    fn read_student(&self, student_id: u64) -> Option<&Student> {
        self.students.get(&student_id)
    }

    // 删除
    fn delete_student(&mut self, student_id: u64) {
        self.students.remove(&student_id);
    }

    // 社团相关操作
    fn create_club(&mut self, id: u64, name: String) {
        let club = Club {
            id,
            name,
        };
        self.clubs.insert(id, club);
    }

    fn update_club(&mut self, club_id: u64, new_name: String) {
        if let Some(club) = self.clubs.get_mut(&club_id) {
            club.name = new_name;
        }
    }

    fn read_club(&self, club_id: u64) -> Option<&Club> {
        self.clubs.get(&club_id)
    }

    fn delete_club(&mut self, club_id: u64) {
        self.clubs.remove(&club_id);
    }

    // 班级相关操作
    fn create_class(&mut self, id: u64, name: String) {
        let class = Class {
            id,
            name,
            students: HashSet::new(),
        };
        self.classes.insert(id, class);
    }

    fn update_class(&mut self, class_id: u64, new_name: String) {
        if let Some(class) = self.classes.get_mut(&class_id) {
            class.name = new_name;
        }
    }

    fn read_class(&self, class_id: u64) -> Option<&Class> {
        self.classes.get(&class_id)
    }

    fn delete_class(&mut self, class_id: u64) {
        self.classes.remove(&class_id);
    }

    // 课程相关操作
    fn create_course(&mut self, id: u64, name: String) {
        let course = Course {
            id,
            name,
        };
        self.courses.insert(id, course);
    }

    fn update_course(&mut self, course_id: u64, new_name: String) {
        if let Some(course) = self.courses.get_mut(&course_id) {
            course.name = new_name;
        }
    }

    fn read_course(&self, course_id: u64) -> Option<&Course> {
        self.courses.get(&course_id)
    }

    fn delete_course(&mut self, course_id: u64) {
        self.courses.remove(&course_id);
    }

        // 学生和课程关联操作
        fn enroll_student_in_course(&mut self, student_id: u64, course_id: u64) {
            if let Some(student) = self.students.get_mut(&student_id) {
                let enrollment = CourseEnrollment {
                    student_id,
                    course_id,
                };
                student.course_enrollments.insert(enrollment);
            }
        }
    
        fn remove_student_from_course(&mut self, student_id: u64, course_id: u64) {
            if let Some(student) = self.students.get_mut(&student_id) {
                let enrollment = CourseEnrollment {
                    student_id,
                    course_id,
                };
                student.course_enrollments.remove(&enrollment);
            }
        }
    
        // 学生和社团关联操作
        fn join_club(&mut self, student_id: u64, club_id: u64) {
            if let Some(student) = self.students.get_mut(&student_id) {
                let membership = ClubMembership {
                    student_id,
                    club_id,
                };
                student.club_memberships.insert(membership);
            }
        }
    
        fn leave_club(&mut self, student_id: u64, club_id: u64) {
            if let Some(student) = self.students.get_mut(&student_id) {
                let membership = ClubMembership {
                    student_id,
                    club_id,
                };
                student.club_memberships.remove(&membership);
            }
        }

        // 学生和班级关联操作
        fn assign_student_to_class(&mut self, student_id: u64, class_id: u64) {
            if let Some(student) = self.students.get_mut(&student_id) {
                student.class_id = class_id;
                if let Some(class) = self.classes.get_mut(&class_id) {
                    class.students.insert(student_id);
                }
            }
        }

        fn unassign_student_from_class(&mut self, student_id: u64) {
            if let Some(student) = self.students.get_mut(&student_id) {
                let class_id = student.class_id;
                student.class_id = 0; // 0 表示未分配班级
                if let Some(class) = self.classes.get_mut(&class_id) {
                    class.students.remove(&student_id);
                }
            }
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
    system.create_club(101, "Programming Club".to_string());
    system.create_class(201, "Computer Science".to_string());
    system.create_course(301, "Rust Programming".to_string());

    // 输出创建的学生、社团、班级、课程信息
    println!("Initial state:");
    println!("Student 1: {:?}", system.read_student(1));
    println!("Club 101: {:?}", system.read_club(101));
    println!("Class 201: {:?}", system.read_class(201));
    println!("Course 301: {:?}", system.read_course(301));

    // 学生和课程关联操作
    system.enroll_student_in_course(1, 301);

    // 输出学生和课程关联后的信息
    println!("\nState after course enrollment:");
    println!("Student 1: {:?}", system.read_student(1));

    // 学生和社团关联操作
    system.join_club(1, 101);

    // 输出学生和社团关联后的信息
    println!("\nState after joining club:");
    println!("Student 1: {:?}", system.read_student(1));

    // 学生和班级关联操作
    system.assign_student_to_class(1, 201);

    // 输出学生和班级关联后的信息
    println!("\nState after assigning student to class:");
    println!("Student 1: {:?}", system.read_student(1));
    println!("Class 201: {:?}", system.read_class(201));



    // // 更新学生、社团、班级、课程信息
    // system.update_student(1, "Alicia".to_string(), 21);
    // system.update_club(101, "Coding Club".to_string());
    // system.update_class(201, "Software Engineering".to_string());
    // system.update_course(301, "Advanced Rust Programming".to_string());

    // // 输出更新后的学生、社团、班级、课程信息
    // println!("\nState after updates:");
    // println!("Updated Student 1: {:?}", system.read_student(1));
    // println!("Updated Club 101: {:?}", system.read_club(101));
    // println!("Updated Class 201: {:?}", system.read_class(201));
    // println!("Updated Course 301: {:?}", system.read_course(301));

    // // 删除学生、社团、班级、课程信息
    // system.delete_student(1);
    // system.delete_club(101);
    // system.delete_class(201);
    // system.delete_course(301);

    // // 输出删除后的学生、社团、班级、课程信息
    // println!("\nState after deletions:");
    // println!("Student 1 after deletion: {:?}", system.read_student(1));
    // println!("Club 101 after deletion: {:?}", system.read_club(101));
    // println!("Class 201 after deletion: {:?}", system.read_class(201));
    // println!("Course 301 after deletion: {:?}", system.read_course(301));
}
