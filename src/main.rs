/* Conway's Quine  */
/* Written by Cole */

#[allow(clippy::too_many_lines)]
fn main() {
    let mut board = "
****************************************************************
*                                                              *
*                                                              *
*                                                              *
*                                                              *
*                             x                                *
*                           x x                                *
*                 xx      xx            xx                     *
*                x   x    xx            xx                     *
*     xx        x     x   xx                                   *
*     xx        x   x xx    x x                                *
*               x     x       x                                *
*                x   x                                         *
*                 xx                                           *
*                                                              *
*                                                              *
*                                                              *
*                                                              *
*                                                              *
*                                                              *
*                                                              *
*                                                              *
*                                                              *
*                                                              *
*                                                              *
*                                                              *
*                                                              *
*                                                              *
*                                                              *
*                                                              *
*                                                              *
****************************************************************
    ".to_string();
    let s = vec![
        "/* Conway's Quine  */",
        "/* Written by Cole */",
        "",
        "#[allow(clippy::too_many_lines)]",
        "fn main() {",
        "    let mut board = \"",
        "    \".to_string();",
        "    let s = vec![",
        "    ];",
        "    let rows = 32;",
        "    let cols = 64;",
        "    let dirs = [(-1, -1),(-1, 0),(-1, 1),",
        "                (0, -1),          (0, 1),",
        "                (1, -1),  (1, 0), (1, 1),];",
        "    let mut board_chars: Vec<char> = board.chars().collect();",
        "    let mut swap_indices: Vec<usize> = vec![];",
        "    for row_idx in 1..rows + 1 {",
        "        for col_idx in 1..cols + 1 {",
        "            let idx = row_idx * (cols + 1) + col_idx;",
        "            if idx >= board.len() {",
        "                break;",
        "            }",
        "            if board_chars[idx] == ' ' || board_chars[idx] == 'x' {",
        "                let mut alive_neighbors = 0;",
        "                for &(row_offset, col_offset) in &dirs {",
        "                    let neighbor_row = row_idx as isize + row_offset;",
        "                    let neighbor_col = col_idx as isize + col_offset;",
        "                    if neighbor_row > 0 && neighbor_row < rows as isize - 1",
        "                        && neighbor_col > 0 && neighbor_col < cols as isize - 1 {",
        "                        let neighbor_index = (neighbor_row as usize) * (cols + 1) + (neighbor_col as usize);",
        "                        if board_chars[neighbor_index] == 'x' {",
        "                            alive_neighbors += 1;",
        "                        }",
        "                    }",
        "                }",
        "                if board_chars[idx] == 'x' {",
        "                    if alive_neighbors != 2 && alive_neighbors != 3 {",
        "                        swap_indices.push(idx);",
        "                    }",
        "                } else if board_chars[idx] == ' ' && alive_neighbors == 3 {",
        "                    swap_indices.push(idx);",
        "                }",
        "            }",
        "        }",
        "    }",
        "    for idx in swap_indices {",
        "        if board_chars[idx] == ' ' {",
        "            board_chars[idx] = 'x';",
        "        } else {",
        "            board_chars[idx] = ' ';",
        "        }",
        "    }",
        "    board = board_chars.into_iter().collect();",
        "    // ==== PRINT CONTENTS ====",
        "    for line in &s[0..6] {",
        "        println!(\"{line}\");",
        "    }",
        "",
        "    let mut board_lines = board.lines();",
        "    board_lines.next(); // skip first empty line in board",
        "",
        "    // print board",
        "    for _ in 0..32 {",
        "        println!(\"{}\", board_lines.next().unwrap_or(\"No line\"));",
        "    }",
        "",
        "    println!(\"{}\", &s[6]);",
        "    println!(\"{}\", &s[7]);",
        "",
        "    // print vec",
        "    for line in &s {",
        "        println!(",
        "            \"        \\\"{}\\\",\",",
        "            line.replace('\\\\', \"\\\\\\\\\").replace('\"', \"\\\\\\\"\")",
        "        );",
        "    }",
        "",
        "    for line in &s[8..s.len()] {",
        "        println!(\"{line}\");",
        "    }",
        "}",
    ];
    let rows = 32;
    let cols = 64;
    let dirs = [(-1, -1),(-1, 0),(-1, 1),
                (0, -1),          (0, 1),
                (1, -1),  (1, 0), (1, 1),];
    let mut board_chars: Vec<char> = board.chars().collect();
    let mut swap_indices: Vec<usize> = vec![];
    for row_idx in 1..rows + 1 {
        for col_idx in 1..cols + 1 {
            let idx = row_idx * (cols + 1) + col_idx;
            if idx >= board.len() {
                break;
            }
            if board_chars[idx] == ' ' || board_chars[idx] == 'x' {
                let mut alive_neighbors = 0;
                for &(row_offset, col_offset) in &dirs {
                    let neighbor_row = row_idx as isize + row_offset;
                    let neighbor_col = col_idx as isize + col_offset;
                    if neighbor_row > 0 && neighbor_row < rows as isize - 1
                        && neighbor_col > 0 && neighbor_col < cols as isize - 1 {
                        let neighbor_index = (neighbor_row as usize) * (cols + 1) + (neighbor_col as usize);
                        if board_chars[neighbor_index] == 'x' {
                            alive_neighbors += 1;
                        }
                    }
                }
                if board_chars[idx] == 'x' {
                    if alive_neighbors != 2 && alive_neighbors != 3 {
                        swap_indices.push(idx);
                    }
                } else if board_chars[idx] == ' ' && alive_neighbors == 3 {
                    swap_indices.push(idx);
                }
            }
        }
    }
    for idx in swap_indices {
        if board_chars[idx] == ' ' {
            board_chars[idx] = 'x';
        } else {
            board_chars[idx] = ' ';
        }
    }
    board = board_chars.into_iter().collect();

    // ==== PRINT CONTENTS ====
    for line in &s[0..6] {
        println!("{line}");
    }

    let mut board_lines = board.lines();
    board_lines.next(); // skip first empty line in board

    // print board
    for _ in 0..32 {
        println!("{}", board_lines.next().unwrap_or("No line"));
    }

    println!("{}", &s[6]);
    println!("{}", &s[7]);

    // print vec
    for line in &s {
        println!(
            "        \"{}\",",
            line.replace('\\', "\\\\").replace('"', "\\\"")
        );
    }

    for line in &s[8..s.len()] {
        println!("{line}");
    }
}
