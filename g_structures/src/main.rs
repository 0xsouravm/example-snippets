fn main() {
    // student_with_arguments(8.5, 20, true, 123456, 2021);
    assign_with_default();
}

// Named Field Struct
struct DummyStudent {
    name: String, // Field
    age: u8, // Field
    is_a_student: bool, // Field
}

// Tuple Structs
struct Color(i32, i32, i32);
struct Point3D(i32, i32, i32);

// let a = Color(255, 0, 0);
// let b = Point3D(0, 0, 0);

// let a = (23, 44, 67);
// let b = (3, 4, 7);

struct Point2D(i32, i32);

// UnitStruct
struct UnitStruct; // Traits and Generics

// Stucts are custom data types that let you group/package together different related
// data and reference them as a single unit.

// Types:
// 1. Named-Field: A struct where all the fields have a name associated to them.
// 2. Tuple-Struct: A struct where fields are defined as a tuple and not named.
// 3. Unit-Struct: A struct where there are no fields defined.


// let tup:  (i32, u8, bool, char) = (500, 5, true, 'a'); // Employee
// let diff: (i32, u8, bool, char) = (500, 5, true, 'a'); // Student


// struct Employee(i32, u8, bool, char);

// let a = Employee(500, 5, true, 'a');
// let b = Employee(500, 5, true, 'a');

// struct CustomStruct(i32, u8, bool, char);

// let custom_tup_struct: CustomStruct = CustomStruct(500, 5, true, 'a');

#[derive(Default, Debug)] // Derives the properties of the Default trait for your 'type'.
struct Student {
    sgpa: f32,
    age: u8, // MAX: 255
    is_a_student: bool,
    sic: u32,
    year: u16
}

fn use_student() {
    let john: Student = Student {
        sgpa: 8.5,
        age: 20,
        is_a_student: true,
        sic: 123456,
        year: 2021
    };

    let jane = Student {
        sgpa: 9.0,
        age: 21,
        is_a_student: true,
        sic: 123457,
        year: 2021,
    };
}

// Short hand notation: If the names of the fields are same as the names of the variables being assigned to them,
// then we can use the short hand notation.
// field_name: variable_name
// if field_name == variable_name, then we can use the short hand notation.
// {
//  sgpa: sgpa, == sgpa
// }

fn student_with_shorthand_notation() {
    // let mut a;
    // let b = 4.5;

    // a = 8.5;
    // a = b; // a stores 4.5
    
    // let student_sgpa = 8.5;
    // let student_age = 20;
    // let is_a_student_var = true;
    // let student_sic = 123456;
    // let student_year = 2021;
    
    // let student = Student {
    //     sgpa: student_sgpa,
    //     age: student_age,
    //     is_a_student: is_a_student_var,
    //     sic: student_sic,
    //     year: student_year
    // };
        
    let sgpa = 8.5;
    let age = 20;
    let is_a_student = true;
    let sic = 123456;
    let year = 2021;

    // let student = Student {
    //     sgpa: sgpa,
    //     age: age,
    //     is_a_student: is_a_student,
    //     sic: sic,
    //     year: year
    // };

    // Short hand notation 
    let student = Student {
        sgpa,
        age,
        is_a_student,
        sic,
        year
    };
}


fn student_with_arguments(sgpa: f32, age: u8, is_a_student: bool, sic: u32, year: u16) {
    let student = Student {
        sgpa,
        age,
        is_a_student,
        sic,
        year
    };
}

fn struct_with_default() {
    let student = Student::default();
    println!("Student: {:?}", student);
}

struct Library {
    name: String,
    location: String,
    books: Book, // Only one book in the library
}

struct Book {
    author: String,
    book_details: BookDetails,
}

struct BookDetails {
    name: String,
    isbn: String,
}

// Implement features/functionalites(methods and associated functions and [trait fns]) for our types.
impl Student {
    // methods - self as their first parameter
    // associated functions - no self as their first parameter

    // constuctor to create a new Student
    fn new(sgpa: f32, age: u8, is_a_student: bool, sic: u32, year: u16) -> Student {
        Student {
            sgpa,
            age,
            is_a_student,
            sic,
            year
        }
    }

    // printing the student's details
    fn print_student_details(&self) {
        // Print details
        self.sgpa;
        self.sic;
        println!("Student Details: ");
    }

    fn compare_student_sgpa(&self, other: Student) {
        if self.sgpa > other.sgpa {
            println!("Self has a higher SGPA");
        }
        else {
            println!("Other has a higher SGPA");
        }
    }
}

fn something_outside_impl() {
    let raj: Student = Student::default();
    let jay: Student = Student {
        sgpa: 9.0,
        age: 20,
        is_a_student: true,
        sic: 123456,
        year: 2021
    };

    let new_student = Student::new(8.5, 20, true, 123456, 2021);
    // raj.compare_student_sgpa(jay);
    jay.compare_student_sgpa(raj);
    // raj.print_student_details();
}
// &self - The instance of the struct on which that particular method is called.
// associated functions are not bound to any instance. They operate on the type itself.

fn assign_with_default() {
    let student_1 = Student::new(8.5, 20, true, 123456, 2021);
    let student_2 = Student {
        sgpa: 4.56,
        ..student_1
    };

    let student_3 = Student {
        sgpa: 6.79,
        age: 46,
        ..Student::default()
    };

    println!("Student 1: {:?}", student_1);
    println!("Student 2: {:?}", student_2);
    println!("Student 3: {:?}", student_3);
}

fn playground() {
    struct Library {
        name: String,
        location: String,
        books: Book, // Only one book in the library
    }
    
    #[derive(Clone)]
    struct Book {
        author: String,
        book_details: BookDetails,
    }
    
    #[derive(Clone)]
    struct BookDetails {
        name: String,
        isbn: String,
    }

    let book_details = BookDetails {
        name: "The Alchemist".to_string(),
        isbn: "1234567890".to_string()
    };

    let book = Book {
        author: "Paulo Coelho".to_string(),
        book_details
    };

    let library = Library {
        name: "Central Library".to_string(),
        location: "Bangalore".to_string(),
        books: book.clone()
    };

    let new_library = Library {
        name: "North Library".to_string(),
        ..library
    };

    let another_new_library = Library {
        books: Book {
            author: "Ruskin Bond".to_string(),
            ..book
        },
        ..new_library
    };
}