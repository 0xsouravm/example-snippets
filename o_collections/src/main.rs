use std::{ collections::{hash_map, HashMap}, vec };
// std::collections::HashMap

fn main() {
    create_hashmap();
}

// Vector 
// Hashmap

// Hashset
// Slices

// Vectors - Dynamically sized 'arrays'. Allocated on the Heap
// [T; N] - Fixed size array of size N. Allocated on the Stack

struct Student;

fn vector_example() {
    let array = [1, 2, 3, 4, 5]; // [i32; 5]
    let same_array = [1; 5]; // [i32; 5]

    let vector = vec![1, 2, 3, 4, 5]; // Vec<i32>
    let same_vector = vec![1; 5]; // Vec<i32>
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);
    vector.push(5);

    // // Type Inference
    // let mut no_type_vector = Vec::new();
    // no_type_vector.push("hello");
}
// push - 6

// 1 2 3 4 5 6

// pop()
// 6

fn vector_pop() {
    let mut vector = vec![1, 2, 3, 4, 5];
    println!("Before Popping: {:?}", vector);

    let last = vector.pop();

    println!("Popped Element: {:?}", last);
    println!("After Popping: {:?}\n", vector);
    // -------------------------------------
    let mut new_vec: Vec<i32> = vec![];
    println!("Before Popping: {:?}", new_vec);

    let last = new_vec.pop();

    println!("Popped Element: {:?}", last);
    println!("After Popping: {:?}", new_vec);
}

fn vector_remove() {
    let mut vector = vec![1, 2, 3, 4, 5];
    println!("Before Removing: {:?}", vector);

    let removed = vector.remove(2);

    println!("Removed Element: {:?}", removed);
    println!("After Removing: {:?}", vector);
    // --------------------------------------------
    let mut vector: Vec<i32> = vec![1, 2, 3];
    println!("Before Removing: {:?}", vector);

    let removed = vector.remove(30);

    println!("Removed Element: {:?}", removed);
    println!("After Removing: {:?}", vector);
}

fn vector_get() {
    let mut vec = vec![1, 2, 3, 4, 5];
    println!("Vector: {:?}", vec);
    // println!("7th Element in the Vector: {:?}", vec[6]);

    // get()
    let third_element = vec.get(3);
    match third_element {
        Some(_) => {
            let _ = vec.remove(3);
        },
        None => println!("No 3rd Element in the Vector"),
    }

    println!("Vector after removing: {:?}", vec);
}

fn vector_contains() {
    let vec = vec![1, 2, 3, 4, 5];
    let contains_13 = vec.contains(&13);
    let contains_3 = vec.contains(&3);
    println!("Vector Contains 13: {:?}", contains_13);
    println!("Vector Contains 3: {:?}", contains_3);
}


fn vector_iteration() {
    let vector = vec![1, 2, 3, 4, 5];
    for element in vector.clone() {
        println!("Element: {}", element);
    }

    println!("\n3rd Element in Vector: {:?}", vector[2]); // why this throws an error?
    // -----------------------------------------------
    let vector = vec![1, 2, 3, 4, 5];
    for element in vector.iter() {
        println!("Element: {}", element);
    }

    println!("\n3rd Element in Vector: {:?}", vector[2]);
}

fn print_str() {
    let a = 100; // String
    let b = &a; // 
    let c = a;  // String

    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
}

// Hashmaps - Dictionaries
// (Key - Value) Pair

// Key -> Mapped to a Value
// "Odisha"(key) -> "Bhubaneswar"(value)
// To access "Bhubaneswar"(value) I would need "Odisha"(key)
// Keys must be unique
// Allocated on the Heap

fn create_hashmap() {
    let mut hash_map = HashMap::new(); // Similar to vector syntax - Vec::new()
    // insert()
    hash_map.insert("Odisha", "Bhubaneswar");
    hash_map.insert("Anything", "Something");

    // Inserting a different key for the same value
    hash_map.insert("Not Anything", "Something");
    
    println!("Map: {:?}", hash_map);
    // Inserting a different value for the same key. This updates the previous value for that key.
    hash_map.insert("Anything", "Different Something");

    println!("Map: {:?}", hash_map);
}

fn hashmap_remove() {
    // remove("Key")
    let mut hash_map = HashMap::new();
    hash_map.insert("Odisha", "Bhubaneswar");
    hash_map.insert("Anything", "Something");
    println!("Map Before Removing: {:?}", hash_map);

    let removed_value = hash_map.remove("Anything");
    let non_existing_removed_value = hash_map.remove("NoKey");

    println!("Removed Value: {:?}", removed_value);
    println!("Non Existing Removed Value: {:?}", non_existing_removed_value);
    println!("Map After Removing: {:?}", hash_map);
}

fn hashmap_get() {
    let mut hash_map = HashMap::new();
    hash_map.insert("Odisha", "Bhubaneswar");
    hash_map.insert("Anything", "Something");
    println!("Map: {:?}", hash_map);

    // get("Key")
    let got_value = hash_map.get("Odisha");
    let non_existing_got_value = hash_map.get("Bihar");

    println!("Got Value: {:?}", got_value);
    println!("Non Existing Got Value: {:?}", non_existing_got_value);
    println!("Map: {:?}", hash_map);
}

fn hashmap_contains() {
    let mut hash_map = HashMap::new();
    hash_map.insert("Odisha", "Bhubaneswar");
    hash_map.insert("Anything", "Something");
    println!("Map: {:?}", hash_map);

    let contains_odisha = hash_map.contains_key("Odisha");
    let contains_bihar = hash_map.contains_key("Bihar");

    println!("Contains Odisha: {:?}", contains_odisha);
    println!("Contains Bihar: {:?}", contains_bihar);
}

fn hashmap_length() {
    let mut hash_map = HashMap::new();
    hash_map.insert("Odisha", "Bhubaneswar");
    hash_map.insert("Anything", "Something");
    hash_map.insert("Jharkhand", "Capital of Jharkhand");
    println!("Map: {:?}", hash_map);

    let length = hash_map.len();
    println!("Length of the Map: {:?}", length);
}

fn hashmap_iterations() {
    let mut hash_map = HashMap::new();
    hash_map.insert("Odisha", "Bhubaneswar");
    hash_map.insert("Anything", "Something");
    hash_map.insert("Jharkhand", "Capital of Jharkhand");
    println!("Map: {:?}", hash_map);

    for (key, value) in hash_map.iter() {
        println!("Key: {}, Value: {}", key, value);
    }

    for (key, value) in &hash_map {
        println!("Key: {}, Value: {}", key, value);
    }

    for (key, value) in hash_map.clone() {
        println!("Key: {}, Value: {}", key, value);
    }

    println!("Map: {:?}", hash_map);

    for key in hash_map.keys() {
        println!("Key: {}", key);
    }

    for value in hash_map.values() {
        println!("Value: {}", value);
    }

    for (key, value) in &hash_map {
        println!("Key: {}, Value: {}", key, value);
    }

    for (key, value) in hash_map {
        println!("Key: {}, Value: {}", key, value);
    }
}