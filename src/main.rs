/* Conway's Quine  */
/* Written by Cole */

fn main() {
    let mut s = "****************
*              *
*              *
*              *
*     xx       *
*     xx       *
*     xx       *
*              *
*              *
*              *
*              *
*              *
*              *
*              *
*              *
****************"
        .to_string();

    let rows = 16;
    let cols = 16;
    let dirs = [
        // (-1, -1), (-1, 0), (-1, 1),
        // ( 0, -1),          ( 0, 1),
        // ( 1, -1), ( 1, 0), ( 1, 1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut s_chars: Vec<char> = s.chars().collect();
    let mut swap_indices: Vec<usize> = vec![];
    for row_idx in 0..rows {
        for col_idx in 0..cols {
            let idx = row_idx * (cols + 1) + col_idx;
            if s_chars[idx] == '*' {
                continue;
            }

            let mut alive_neighbors = 0;
            for &(row_offset, col_offset) in &dirs {
                let neighbor_row = row_idx as isize + row_offset;
                let neighbor_col = col_idx as isize + col_offset;

                if neighbor_row > 0
                    && neighbor_row < rows as isize - 1
                    && neighbor_col > 0
                    && neighbor_col < cols as isize - 1
                {
                    let neighbor_index =
                        (neighbor_row as usize) * (cols + 1) + (neighbor_col as usize);
                    if s_chars[neighbor_index] == 'x' {
                        alive_neighbors += 1;
                    }
                }
            }

            if s_chars[idx] == 'x' {
                if alive_neighbors != 2 && alive_neighbors != 3 {
                    swap_indices.push(idx);
                }
            } else if s_chars[idx] == ' ' && alive_neighbors == 3 {
                swap_indices.push(idx);
            }
        }
    }

    for idx in swap_indices {
        if s_chars[idx] == ' ' {
            s_chars[idx] = 'x';
        } else {
            s_chars[idx] = ' ';
        }
    }
    s = s_chars.into_iter().collect();
    println!("{s}");
}
