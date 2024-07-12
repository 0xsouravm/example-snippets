
#[cfg(test)]
mod xyz{
    use crate::{add, divide};

    #[test]
    fn divide_works_successfully(){
        assert_eq!(divide(6, 3), Some(2));
    }


    // #[test]
    // #[should_panic] // if there is error then it's intensional and the test will pass
    // fn divide_fails(){
    //     assert_eq!(add(5 ,2), 7);
    //     // assert_eq!(divide(6, 2), Some(2));
    //     // assert_ne!(divide(6, 0), Some(6));
    //     // assert_ne!(divide(6, 0), Some(4));
    // }


    #[test]
    fn add_works_successfully(){
        let result = add(2, 4);
        if result != 5 {
            panic!("Expected 5, but got {}", result);
        }
    }

    // #[test]
    fn assert_works(){
        let x = 5;
        assert!(x==6);
        assert_eq!(x, 6);
    }

    // #[test]
    // fn add_works(){
    //     assert_eq!(5, 5);
    //     // assert_eq!(add(2, 3), 5);
    // }
}



// assert_eq is a macro
// checks if the values passed to it are equal or not


// assert!
// assert_eq!
// assert_ne!
// panic!


// assert!(add(2, 3));

// assert works for true values and fails for false


// assert_ne is opposite of assert_eq