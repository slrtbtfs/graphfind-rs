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

// Defined Traits/Node Types
/**
 * Person Trait/abstract; only use professors or students.
 */
pub trait Person {
    fn name(&self) -> &String;
    fn age(&self) -> u32;
}

/**
 * Student Trait, inherits from Person
 *
 * Defines a Constructor/Associated Function.
 */
pub trait Student: Person {
    // Fields
    fn matrical_number(&self) -> u32;
}

/**
 * Professor Trait
 */
pub trait Professor: Person {
    // Fields
    fn faculty(&self) -> &String;
}

// Structs/Data Containers.
/**
 * Student struct
 */
#[derive(Debug)]
pub struct StudentStruct {
    name: String,
    age: u32,
    matrical_number: u32,
}

/**
 * Student Struct
 */
#[derive(Debug)]
pub struct ProfessorStruct {
    name: String,
    age: u32,
    faculty: String,
}

/**
 * FriendOf Struct
 */
#[derive(Debug)]
pub struct FriendOf {
    since_year: i32,
}

// Implementations of Node Types
impl Person for StudentStruct {
    fn name(&self) -> &String {
        &self.name
    }

    fn age(&self) -> u32 {
        self.age
    }
}

impl Student for StudentStruct {
    fn matrical_number(&self) -> u32 {
        self.matrical_number
    }
}

impl Person for ProfessorStruct {
    fn name(&self) -> &String {
        &self.name
    }

    fn age(&self) -> u32 {
        self.age
    }
}

impl Professor for ProfessorStruct {
    fn faculty(&self) -> &String {
        &self.faculty
    }
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
pub fn new_student(name: &str, age: u32, matrical_number: u32) -> StudentStruct {
    StudentStruct {
        name: String::from(name),
        age,
        matrical_number,
    }
}

pub fn new_professor(name: &str, age: u32, faculty: &str) -> ProfessorStruct {
    ProfessorStruct {
        name: String::from(name),
        age,
        faculty: String::from(faculty),
    }
}
