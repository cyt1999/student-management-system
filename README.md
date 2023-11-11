输出
```
Initial state:
Student 1: Some(Student { id: 1, name: "Alice", age: 20, class_id: 101, club_ids: {}, course_enrollments: {}, club_memberships: {} })
Club 101: Some(Club { id: 101, name: "Programming Club" })
Class 201: Some(Class { id: 201, name: "Computer Science", students: {} })
Course 301: Some(Course { id: 301, name: "Rust Programming" })

State after course enrollment:
Student 1: Some(Student { id: 1, name: "Alice", age: 20, class_id: 101, club_ids: {}, course_enrollments: {CourseEnrollment { student_id: 1, course_id: 301 }}, club_memberships: {} })

State after joining club:
Student 1: Some(Student { id: 1, name: "Alice", age: 20, class_id: 101, club_ids: {}, course_enrollments: {CourseEnrollment { student_id: 1, course_id: 301 }}, club_memberships: {ClubMembership { student_id: 1, club_id: 101 }} })

State after assigning student to class:
Student 1: Some(Student { id: 1, name: "Alice", age: 20, class_id: 201, club_ids: {}, course_enrollments: {CourseEnrollment { student_id: 1, course_id: 301 }}, club_memberships: {ClubMembership { student_id: 1, club_id: 101 }} })
Class 201: Some(Class { id: 201, name: "Computer Science", students: {1} })
```
