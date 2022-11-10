/**
 * Defines the data of a test graph:
 *
 * - Nodes:
 *  - Person (name, age)
 *  - Student (extends Student, matrical number)
 *  - Professor (extends Student, faculty)
 * - Edges:
 *  - FriendOf (since year)
 */

// Defined Node Types
/**
 * Person enum/Uses redundant data for now.
 */

#[derive(Debug, PartialEq, Eq)]
pub enum Person {
    Student{name: String, age: u32, matrical_number: u32},
    Professor{name: String, age: u32, faculty: String},
}

/**
 * FriendOf Struct
 */
#[derive(Debug, PartialEq, Eq)]
pub struct FriendOf {
    since_year: i32,
}

// Implementation of Edge Type
impl FriendOf {
    pub fn new(year: i32) -> FriendOf {
        FriendOf { since_year: year }
    }

    pub fn since_year(&self) -> i32 {
        self.since_year
    }
}

// Factory Methods
pub fn new_student(name: &str, age: u32, matrical_number: u32) -> Person {
    Person
::Student {
        name: String::from(name),
        age,
        matrical_number,
    }
}

pub fn new_professor(name: &str, age: u32, faculty: &str) -> Person {
    Person
::Professor {
        name: String::from(name),
        age,
        faculty: String::from(faculty),
    }
}
