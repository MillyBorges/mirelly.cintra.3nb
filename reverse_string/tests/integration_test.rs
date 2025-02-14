use reverse_string::reverse_string;

#[test]
fn test_reverse_string_normal(){
    let input = "Data Structures";
    let expected = "serutcurtS ataD";
    assert_eq!(reverse_string(input), expected)
}

#[test]
fn test_reverse_string_empty(){
    let input = "";
    let expected = "";
    assert_eq!(reverse_string(input), expected)
}

#[test]
fn test_reverse_string_caractere_unico(){
    let input = "A";
    let expected = "A";
    assert_eq!(reverse_string(input), expected)
}