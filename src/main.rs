use std::collections::VecDeque;

fn print_matrix(matrix: [[&str; 5]; 5]) {
    for x in &matrix {
        let mut display_row = " ".to_owned();
        for y in x {
            display_row.push_str(y.to_string().as_ref());
            display_row.push_str(" ");
        }
        println!("{}", display_row);
    }
}

fn depth_first_search(matrix: [[&str; 5]; 5], start: (usize, usize)) {
    let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
    stack.push_back(start);
    let (current_node_row, current_node_column) = stack.pop_back().unwrap();
    println!(
        "row -> {:?}  node -> {:?} ",
        current_node_row, current_node_column
    );
    let mut tito = "11";
    tito = "12";
    tito ="151515";
    println!("{:?}", matrix.len());
    println!("{:?}", stack);
}

fn is_valid_node(matrix: [[&str; 5]; 5], row: usize, column: usize) -> bool {
    if column >= 0
        && row >= 0
        && column < matrix.len()
        && row < matrix.len()
        && matrix[row][column].parse::<i32>().unwrap() == 0
    {
        true
    } else {
        false
    }
}

fn main() {
    let matrix: [[&str; 5]; 5] = [
        ["0", "1", "1", "0", "0"],
        ["1", "1", "0", "0", "0"],
        ["1", "0", "0", "1", "0"],
        ["1", "1", "0", "1", "1"],
        ["0", "1", "0", "0", "0"],
    ];
    print_matrix(matrix);
    depth_first_search(matrix, (0, 4));
}
