use std::collections::VecDeque;

fn print_matrix(matrix: [[&str; 5]; 5]) {
    for x in &matrix {
        let mut display_row = " ".to_owned();
        for y in x {
            display_row.push_str(y.to_string().as_ref());
            display_row.push(' ');
        }
        println!("{}", display_row);
    }
}

fn depth_first_search(mut matrix: [[&str; 5]; 5], start: (isize, isize)) -> [[&str; 5]; 5] {
    let mut stack: VecDeque<(isize, isize)> = VecDeque::new();
    let path_traveled: Vec<(isize, isize)> = Vec::new();

    stack.push_back(start);
    while ! stack.is_empty() {
        let (current_node_row, current_node_column) = stack.pop_back().unwrap();
        matrix[current_node_row as usize][current_node_column as usize] = "X";
        for valid_next_nodes in valid_next_nodes(
            matrix,
            (current_node_row, current_node_column),
            path_traveled.to_owned()
        ) {
            stack.push_back(valid_next_nodes);
        }
    }
    matrix
}

fn valid_next_nodes(
    matrix: [[&str; 5]; 5],
    current_pos: (isize, isize),
    path_traveled: Vec<(isize, isize)>,
) -> Vec<(isize, isize)> {
    // UP, DOWN, LEFT, RIGHT
    let mut valid_next_nodes = Vec::new();
    let directions_offset: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let (current_node_row, current_node_column) = current_pos;
    for (row_offset, column_offset) in directions_offset.iter() {
        let (next_row, next_col) = (
            current_node_row + row_offset,
            current_node_column + column_offset,
        );
        if is_valid_node(matrix, next_row, next_col)
            && !path_traveled.contains(&(next_row, next_col))
        {
            valid_next_nodes.push((next_row, next_col));
        }
    }
    valid_next_nodes
}

fn is_valid_node(matrix: [[&str; 5]; 5], row: isize, column: isize) -> bool {
     column >= 0
        && row >= 0
        && (column as usize) < matrix.len()
        && (row as usize) < matrix.len()
        && matrix[row as usize][column as usize]
            == "0"
}

fn main() {
    let mut matrix: [[&str; 5]; 5] = [
        ["0", "1", "1", "0", "0"],
        ["1", "1", "0", "0", "0"],
        ["1", "0", "0", "1", "0"],
        ["1", "1", "0", "1", "1"],
        ["0", "1", "0", "0", "0"],
    ];
    print_matrix(matrix);
    println!("\n");
    matrix = depth_first_search(matrix, (0, 4));
    print_matrix(matrix);
}
