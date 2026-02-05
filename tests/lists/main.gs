costumes "blank.svg";

struct MyStruct {
    name,
    date,
}

list MyStruct my_struct_list;

proc main {
    # Test basic struct field access
    add MyStruct { name: "Alice", date: "01/01/2026" } to my_struct_list;
    add MyStruct { name: "Bob", date: "02/02/2026" } to my_struct_list;
    add MyStruct { name: "Charlie", date: "03/03/2026" } to my_struct_list;
    
    # Test struct field index query
    index = "02/02/2026" in my_struct_list.date;
    say "Index: " & index;
    
    # Test another field
    has_alice = "Alice" in my_struct_list.name;
    say "Has Alice: " & has_alice;
    
    # Test non-existent value
    has_nonexistent = "04/04/2026" in my_struct_list.date;
    say "Has non-existent date: " & has_nonexistent;
}

onflag {
    main;
}