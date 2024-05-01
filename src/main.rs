use raylib::prelude::*;
use raylib::ffi::KeyboardKey::*;
use raylib::ffi::MouseButton::*;
use raylib::ffi::CheckCollisionPointRec;


//use std::ops::Add;
//use raylib::core::texture::Image;

const WIDTH:f32 = 600.0;
const HEIGHT:f32 = 400.0;
struct BALL{
    position:Vector2,
    speed:f32,
    radius:f32,
    color:Color
}

fn draw_text(
    d:&mut RaylibDrawHandle,
    text:&str,
    y:i32,
    font_size:i32,
    color:Color 
){
    let text_length = measure_text(text,font_size);
    d.draw_text(
        text,
        WIDTH as i32 / 2 - (text_length / 2),
        y,
        font_size,
        color
    )
}


pub fn check_point_inside_rect(point: Vector2, rec: Rectangle) -> bool {
    unsafe { CheckCollisionPointRec(point.into(), rec.into()) }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Hello univers_anouar")
        .build();

    let mut ball = BALL{
        position:Vector2::new( WIDTH /2.0, 100.0),
        speed:3.0,
        radius:40.0,
        color:Color::GREEN
    };
    rl.set_target_fps(60);

    let mut frame_count:i32 = 1;
    let mut value:i32 = 0;
    let file_name = "images/univers.png".to_string();


    //let img = Image::load_image(&file_name);
    let image = rl.load_texture(&thread,&file_name).unwrap();
    println!("{:?}",image);

    //create vector to move texture in canvas
    let mut pos_tex_vec ;
    



    while !rl.window_should_close() {

// key board pressed

        //is_key_down
        //is_key_up
        //is_key_pressed

        if rl.is_key_pressed(KEY_RIGHT) {ball.position.x += ball.speed;}
        if rl.is_key_pressed(KEY_LEFT) {ball.position.x -= ball.speed;}
        if rl.is_key_pressed(KEY_UP) {ball.position.y += ball.speed;}
        if rl.is_key_pressed(KEY_DOWN) {ball.position.y -= ball.speed;}

// end key bord pressed
pos_tex_vec = rl.get_mouse_position();
if rl.is_mouse_button_down(MOUSE_LEFT_BUTTON){
    ball.position = ball.position.lerp(
        rl.get_mouse_position(),
        0.05 //more this value is less more the movement in smoth
    );
}
//mouse pressed


//end mouse pressed


        let button_rec1 = Rectangle{x:500.0, y:100.0, width:100.0, height:50.0};
        let mut button_color = Color::BLUE;
        if check_point_inside_rect(rl.get_mouse_position(),button_rec1) {
            button_color = Color::ORANGE;
        }                  

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);


//##########################draw text

frame_count += 1;
if frame_count % 60 == 0 {
value = get_random_value::<i32>(-100,100);
frame_count = 0;
}
draw_text(&mut d,"test add text",200,20,Color::YELLOW);
draw_text(&mut d,format!("{}",value).as_str(),300,30,Color::YELLOW);

//##########################end draw text


//##########################load images

//d.draw_texture_v(&image,Vector2::new(15.0,15.0),Color::WHITE);
    
    d.draw_texture_pro(
    &image,
    Rectangle{x:0.0,
    y:0.0,
    width:512.00,
    height:512.00},
    Rectangle{
    x:pos_tex_vec.x,
    y:pos_tex_vec.y,
    width:100.00,
    height:100.00
    },
    Vector2::new(50.0,50.0),
    0.0,
    Color::WHITE);



//##########################end load images

//##########################button


d.draw_rectangle_rec(button_rec1,button_color);
d.draw_text("click me",
( button_rec1.x + button_rec1.width/2.0 - (measure_text("click me",20)/2) as f32) as i32,
(button_rec1.y + button_rec1.height/2.0 - 20.0/2.0 )as i32,
 20 ,
  Color::BLACK
);
//##########################button

//##########################text
d.draw_text("! hello there ", 12, 12, 20, Color::WHITE);
//##########################end text

//##########################shape
d.draw_circle_v(ball.position, ball.radius, ball.color);
//##########################end shape

}
}