mod drawing;
mod file;
extern crate minilibx;

use std::process;
use minilibx::{Mlx, MlxError, MlxWindow};







// Window could be resized
static WIDTH: i32 = 1080;
static HEIGHT: i32 = 720;
const TITLE: &str = "Wireframe Map";









fn setup() -> Result<(Mlx, MlxWindow), MlxError> {

    let mlx = Mlx::new().unwrap();
    let window = mlx.new_window(WIDTH, HEIGHT, TITLE).unwrap();

    return Ok((mlx, window));
}

fn handle_hooks(mut mlx: Mlx, window: MlxWindow) {
    window.key_hook(
        move |keycode, _| {
            println!("Key Pressed - {}", keycode);


            
            if keycode == 65307 { //ESC Key
                process::exit(0);

            } else if keycode == 97 {
                let x = WIDTH / 2;
                let y = HEIGHT / 2;
                let color = 0xf3f3f3;
                mlx.pixel_put(&window, x, y, color);

                drawing::draw_line(&mlx, &window, 0,0,500,500, None);


            }  else if keycode == 111 { // 'o' key to open the file dialog
                let contents = file::open_and_read_fdf_file();

                match contents{
                    Some(contents) =>{
                        //println!("File contents: {}", contents);

                        let matrix = file::populate_matrix(contents);

                        file::print_matrix(&matrix);
                    },
                    None => {
                        println!("No file Correctly Selected");
                    }
                }
                


                

            }
        },
        &(),
    );


    // Close the window when the 'X' button is clicked
    //window.close_hook(move || {
    //    process::exit(0);
    //});
}






//DRa


fn main() {
    println!("Hello, world!");



   
    let setup_result = setup();
    let (mlx, window);
    
    match setup_result {
        Ok((m, w)) => {
            mlx = m;
            window = w;
        },
        Err(error) => {
            eprintln!("Error setting up MiniLibX: {:?}", error);
            process::exit(1);
        }
    };

    handle_hooks(mlx, window);



    // this will loop forever
    mlx.event_loop();

}
