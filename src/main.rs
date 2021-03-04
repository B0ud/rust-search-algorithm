

pub fn print_matrix(matrix : [[i32; 5]; 5])  {
    for x in &matrix {
        let mut display_row  = " ".to_owned();
        for y in x {
            display_row.push_str(y.to_string().as_ref());
            display_row.push_str(" ");
        }
        println!("{}", display_row);gio:wq:qqqq
    }

}


fn main() {
    let matrix : [[i32; 5]; 5] = [
        [0, 1, 1, 0, 0],
        [1, 1, 0, 0, 0],
        [1, 0, 0, 1, 0],
        [1, 1, 0, 1, 1],
        [0, 1, 0, 0, 0],
    ];
    print_matrix(matrix);
}
