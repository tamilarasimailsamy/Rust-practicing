// Enum defined outside the main function
enum SpreadSheetCell {
    Integer(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut row = vec![
        SpreadSheetCell::Integer(34),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("Blue")),
    ];

    // Example to print values
    for cell in &row {
        match cell {
            SpreadSheetCell::Integer(value) => println!("Integer: {}", value),
            SpreadSheetCell::Float(value) => println!("Float: {}", value),
            SpreadSheetCell::Text(value) => println!("Text: {}", value),
        }
    }
}
