use std::fs::File;
//use std::io;
use std::io::Read;
use rfd::FileDialog;



pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}


pub fn open_and_read_fdf_file() -> Option<String> {
    // Open a file dialog and restrict selection to .fdf files
    let file_dialog = FileDialog::new()
        .add_filter("FDF files", &["fdf"]); // Restrict to .fdf files

    // Show the file dialog and let the user pick a file
    let file_path = file_dialog.pick_file(); // Opens the file dialog

    // Check if a file was selected
    match file_path {
        Some(path) => {
            // Try to open the selected file
            let open_result = File::open(&path);

            // Check if the file was successfully opened
            match open_result {
                Ok(mut file) => {
                    let mut contents = String::new();

                    // Try to read the file's contents into the string
                    let read_result = file.read_to_string(&mut contents);

                    // Check if reading the file was successful
                    match read_result {
                        Ok(_) => {
                            return Some(contents);
                        },
                        Err(_) => {
                            return None; // Return None if reading failed
                        }
                    }
                }
                Err(_) => {
                    return None; // Return None if opening the file failed
                }
            }
        }
        None => {
            return None; // Return None if no file was selected
        }
    }
}




pub fn populate_matrix(file_content: String) -> Vec<Vec<Point>> {


    let mut matrix = Vec::new();


    for (y, line) in file_content.lines().enumerate() {
        let mut row = Vec::new();


        for (x, z_str) in line.split_whitespace().enumerate() {

            let z_int = z_str.parse::<i32>();

            match z_int {


                Ok(z_int)=>{

                    let point = Point {
                        x: x as i32, 
                        y: y as i32, 
                        z: z_int       
                    };

                    row.push(point);

    

                },
                Err(err) =>{
                    println!("Error populate matrix - {}", err)
                }

            }


        }

        matrix.push(row);


    }

    return matrix;

}




pub fn print_matrix(matrix: &Vec<Vec<Point>>) {
    for row in matrix.iter() {
        for point in row.iter() {
            // Print each Point in the format: (x, y, z)
            print!("({},{},{}) ", point.x, point.y, point.z);
        }
        // Print a newline after each row
        println!();
    }
}