use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct MpsData {
    name: String,
    rows: Vec<Row>,
    columns: HashMap<String, Vec<(String, f64)>>,
    rhs: Option<HashMap<String, Vec<(String, f64)>>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RowType {
    N,  // Objective function
    G,  // Greater-than constraint
    L,  // Less-than constraint
    E,  // Equality constraint
}

#[derive(Debug)]
struct Row {
    row_type: RowType,
    name: String,
}

fn parse_mps_file<P: AsRef<Path>>(path: P) -> io::Result<MpsData> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();

    let mut name = String::new();
    let mut rows = Vec::new();
    let mut columns: HashMap<String, Vec<(String, f64)>> = HashMap::new();
    let mut rhs: Option<HashMap<String, Vec<(String, f64)>>> = None;
    while let Some(Ok(line)) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        match trimmed {
            l if l.starts_with("NAME") => {
                name = l.split_whitespace().nth(1).unwrap_or_default().to_string();
            }
            _ => {}
        }
        match trimmed {
            l if l.starts_with("ROWS") => {
                parse_rows(&mut lines, &mut rows)?;
                parse_columns(&mut lines, &mut columns)?;
                rhs = Some(parse_rhs(&mut lines)?);
                
                
            }
            _ => {}
        }

    }

    Ok(MpsData {
        name,
        rows,
        columns,
        rhs,
    })
}

fn parse_rows<I: Iterator<Item = io::Result<String>>>(lines: &mut I, rows: &mut Vec<Row>)-> io::Result<()>{
    for line in lines {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.starts_with("COLUMNS") {
            
            break;
        }
        let mut parts = trimmed.split_whitespace();
        if let (Some(row_type), Some(name)) = (parts.next(), parts.next()) {
            let row_type = match row_type {
                "N" => RowType::N,
                "G" => RowType::G,
                "L" => RowType::L,
                "E" => RowType::E,
                _ => continue,
            };
            rows.push(Row {
                row_type,
                name: name.to_string(),
            });
        }
    }
    
    Ok(())
}

fn parse_columns<I: Iterator<Item = io::Result<String>>>(
    lines: &mut I,
    columns: &mut HashMap<String, Vec<(String, f64)>>,
) -> io::Result<()> {

    for line in lines {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.starts_with("RHS") {
            break;
        }
        
        let parts: Vec<_> = trimmed.split_whitespace().collect();
        if parts.len() >= 3 {
            let column_name = parts[0].to_string();
            let row_name = parts[1].to_string();
            let value: f64 = parts[2].parse().unwrap_or(0.0);

            columns.entry(column_name).or_default().push((row_name, value));
        }
    }
    Ok(())
}

fn parse_rhs<I: Iterator<Item = io::Result<String>>>(
    lines: &mut I,
) -> io::Result<HashMap<String, Vec<(String, f64)>>> {
    let mut rhs: HashMap<String, Vec<(String, f64)>> = HashMap::new();
    for line in lines {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.starts_with("BOUNDS") || trimmed.is_empty() {
            break;
        }

        let parts: Vec<_> = trimmed.split_whitespace().collect();
        if parts.len() >= 3 {
            let rhs_name = parts[0].to_string();
            let row_name = parts[1].to_string();
            let value: f64 = parts[2].parse().unwrap_or(0.0);

            rhs.entry(rhs_name).or_default().push((row_name, value));
        }
    }
    Ok(rhs)
}

fn convert_to_matrices(data: &MpsData) -> (Vec<f64>, Vec<Vec<f64>>, Vec<f64>) {
    let mut objective = vec![0.0; data.columns.len()]; // Initialize c vector with zeros
    let mut constraint_matrix = vec![]; // A matrix
    let mut rhs_vector = vec![0.0; data.rows.len() - 1]; // b vector
    let mut constraint_types = vec![]; // RowType vector

    // Map row names to indices
    let row_indices: HashMap<_, _> = data.rows.iter().enumerate().map(|(i, row)| (&row.name, i)).collect();

    // Initialize constraint matrix rows
    for row_val in &data.rows {
        if row_val.name != "OBJ"{
            constraint_matrix.push(vec![0.0; data.columns.len()]);
        }
    }

    // Populate the objective function and constraint matrix
    let mut column_index = 0;
    for (_, entries) in &data.columns {
        for (row_name, value) in entries {
            if let Some(&row_index) = row_indices.get(row_name) {
                match data.rows[row_index].row_type {
                    RowType::N => {
                        objective[column_index] = *value;
                    }
                    _ => {
                        constraint_matrix[row_index - 1][column_index] = *value;
                        
                        
                    }
                }
            }
        }
        column_index += 1; 
    }

    // Extract RHS values
    if let Some(rhs_data) = &data.rhs {
        for (_rhs_name, entries) in rhs_data {
            for (row_name, value) in entries {
                if let Some(&row_index) = row_indices.get(row_name) {
                    if !row_name.starts_with("OBJ"){
                        rhs_vector[row_index - 1] = *value;
                    }
                    
                }
            }
        }
    }

    // Capture constraint types
    for row in &data.rows {
        if row.name != "OBJ"{
            constraint_types.push(row.row_type);
        } 
    }

    // Make all E constraints <=
    let mut row_index = 0;
    for row in &data.rows {
        if row.row_type == RowType::E{
            constraint_types[row_index - 1] = RowType::L;
            // copy a row and make the copy G
            rhs_vector.push(rhs_vector[row_index - 1]);
            constraint_matrix.push(constraint_matrix[row_index - 1].clone());
            constraint_types.push(RowType::G);
        } 
        row_index += 1;
    }

    // Make all G constraints <=
    row_index = 0;
    for row_type in constraint_types {
        if row_type == RowType::G{
            // multiply by -1
            rhs_vector[row_index] = rhs_vector[row_index] - rhs_vector[row_index] - rhs_vector[row_index];
            for column_index in 0..constraint_matrix[row_index].len(){
                // multiply entry by -1
                constraint_matrix[row_index][column_index] = constraint_matrix[row_index][column_index] - constraint_matrix[row_index][column_index] - constraint_matrix[row_index][column_index];
            }
        } 
        row_index += 1;
    }

    (objective, constraint_matrix, rhs_vector)
}

fn main() -> io::Result<()> {
    let mps_data = parse_mps_file("test/example.mps")?;
    let (c, a, b) = convert_to_matrices(&mps_data);
    println!("[+] Sucsessfully read problem {:?}", mps_data.name);
    println!("[+] Objective Function (c): {:?}", c);
    println!("[+] Constraint Matrix (A): {:?}", a);
    println!("[+] RHS Vector (b): {:?}", b);

    Ok(())
}

