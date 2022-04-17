use raylib::prelude::*;      
use euler::{Vec2,vec2,Vec3,vec3};
use num_complex::Complex32;

const origin : Vec2 = Vec2{ x :  800.0, y : 450.0};   
const screen : Vec2 = Vec2{ x : 1600.0, y : 900.0};  
const scale : f32 = 100.0;
const I : Complex32 = Complex32::new(0.0,1.0);
const pi2 : f32 = 2.0 * PI as f32;

fn from_complex(c : Complex32) -> Vector2 {
    Vector2{ x : origin.x + c.re * scale, y : origin.y - c.im * scale}
}

fn to_complex(v : Vec2) -> Complex32 {
    Complex32::new((v.x - origin.x) / scale,(origin.y - v.y) / scale)
}

struct Fourier {
    n : usize,
    F : Vec<Complex32>,
    C : Vec<Complex32>,
    P : Vec<Complex32>, 
}

// c_n = integral from 0 to 1 (-2 pi I n t).exp() f(t)  dt
impl Fourier {
    fn compute(&mut self) {
        self.C =  vec!(Complex32::new(0.0,0.0); 2 * self.n + 1);
        for i in 0..(2 * self.n + 1) {
            self.C[i] = self.integral(i as i32 -self.n as i32);
        }
    }
    fn integral(&self,n : i32) -> Complex32 {
        let mut res = Complex32::new(0.0,0.0);
        for i in 0..self.F.len() {
            res += self.F[i] * (-pi2 * I * n as f32 * (i as f32 / self.F.len() as f32)).exp() / self.F.len() as f32;
        }
        res
    }
    fn out(&self,t : f32) -> Complex32 {
        let mut res = Complex32::new(0.0,0.0);
        for i in 0..self.C.len() {
            res += self.C[i] * (pi2 * I * (i as f32 - self.n as f32) * t).exp();
        }
        res
    }
    fn to_points(&mut self) {
        self.P = vec!();
        let Num : usize = 1000;
        for m in 0..Num {
            self.P.push(self.out(m as f32 / Num as f32));
        }
    }
}

fn main() {   
    let (mut rl, thread) = raylib::init()
        .size(screen.x as i32, screen.y as i32)
        .title("Collisions")
        .build();
    let backgroung = Color::BLACK;
    
    let mut P : Vec<Vec2> = vec!();
    let mut C : Vec<Complex32> = vec!();
   
    let mut key_presed = false;
    let mut was_drawing = false;
    let mut draw_fourier = false;

    let mut pos : Vec2 = vec2!();

    let mut F : Fourier = Fourier{n : 10, F : vec!(), C : vec!(), P : vec!()};
    let mut t : f32 = 0.0;
    let mut k : i32;
    let mut base  : Complex32;
    let mut base1 : Complex32;
    let mut CC : Complex32;

    let mut draw_color;

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        match rl.get_key_pressed() {
            Some(KeyboardKey::KEY_R) => {key_presed = false; was_drawing = false; draw_fourier = false; P = vec!(); C = vec!(); F = Fourier{n : 10, F : vec!(), C : vec!(), P : vec!()};}
            Some(KeyboardKey::KEY_C) => {F.F = C.clone(); F.compute(); F.to_points(); draw_fourier = true; t = 0.0;}
            Some(KeyboardKey::KEY_UP)   => {if F.n != 100 {F.n += 1; F.F = C.clone(); F.compute(); F.to_points(); draw_fourier = true; t = 0.0;}}
            Some(KeyboardKey::KEY_DOWN) => {if F.n !=   0 {F.n -= 1; F.F = C.clone(); F.compute(); F.to_points(); draw_fourier = true; t = 0.0;}}
           _ => {}
        }
        match rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            true => {key_presed = true; println!("you pressed key");}
            _ => {}
        }
        if !was_drawing && key_presed {
            match rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                true => {println!("you are drawing"); pos = vec2!(rl.get_mouse_x() as f32,rl.get_mouse_y() as f32); P.push(pos); C.push(to_complex(pos)); }
                _ => {was_drawing = true;}
            }
        }
        if was_drawing {
            println!("you finished drawing");
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(backgroung);

        if draw_fourier == true {draw_color = Color::from((255,0,0,100));} else {draw_color = Color::RED;}
        if P.len() != 0 {
            for p in 0..P.len() {
                d.draw_line_ex(Vector2{ x :               P[p].x, y :               P[p].y},
                               Vector2{ x : P[(p+1) % P.len()].x, y : P[(p+1) % P.len()].y},2.0,draw_color);
            } 
        }
        
        d.draw_fps(10,10);

        if draw_fourier {
            for p in 0..(F.P.len() as f32 * t) as usize {
                d.draw_line_ex(from_complex(F.P[p]),
                               from_complex(F.P[(p+1) % F.P.len()]),2.0,Color::YELLOW);
            }
            d.draw_text("fourier transform comleted",150,10,20,Color::WHITE);
            d.draw_text(&format!("{} vectors",2 * F.n)[..],500,10,20,Color::WHITE);
            base = Complex32::new(0.0,0.0);
            for i in 0..F.C.len() as i32 {
                if i != 0 {
                    if i % 2 == 0 {k = -(i+1) / 2;} else {k = (i+1) / 2;}
                } else {k = 0;}
                CC = F.C[(F.n as i32 + k) as usize];
                base1 = base + CC * (pi2 * I * k as f32 * t).exp();
                if i != 0 {
                    d.draw_line_ex(from_complex(base),from_complex(base1),2.0,Color::WHITE);
                    d.draw_circle_lines(from_complex(base).x as i32,from_complex(base).y as i32,(CC.re.powi(2) + CC.im.powi(2)).sqrt() * scale,Color::GRAY);
                }
                base = base1;
            }
        }
        t += 0.001;
        if t > 1.0 {t = 0.0;}
    }
}
