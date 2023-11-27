use ris_util::io::*;

#[test]
fn should_compare_bytes() {
    let array1 = [1, 2, 3];
    let array2 = [1, 2, 3];
    let array3 = [];
    let array4 = [];
    let array5 = [1, 2, 4];
    let array6 = [1, 2];
    let array7 = [1, 2, 3, 4];
    let array8 = [4, 5, 6];

    assert!(bytes_equal(&array1, &array2));
    assert!(!bytes_equal(&array1, &array3));
    assert!(!bytes_equal(&array1, &array4));
    assert!(!bytes_equal(&array1, &array5));
    assert!(!bytes_equal(&array1, &array6));
    assert!(!bytes_equal(&array1, &array7));
    assert!(!bytes_equal(&array1, &array8));
    assert!(bytes_equal(&array3, &array4));
}
