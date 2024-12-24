use speedy2d::{Window, Graphics2D};
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};

mod vector {
    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector {
            Vector { x, y }
        }

        pub fn add(&mut self, other: &Vector) {
            self.x += other.x;
            self.y += other.y;
        }

        pub fn set(&mut self, x: f32, y: f32) {
            self.x = x;
            self.y = y;
        }
    }
}

use vector::Vector;

fn main() {
    let window = Window::new_centered("Pendulum", (800, 480)).unwrap();

    let win = MyWindowHandler {
        p: Pendulum::new(400.0, 0.0, 200.0),
    };

    window.run_loop(win);
}

struct MyWindowHandler {
    p: Pendulum,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {

        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        self.p.update();
        self.p.draw(graphics);


        helper.request_redraw();
    }
}

struct Pendulum {
    origin: Vector,
    position: Vector,
    angle: f32,
    angular_velocity: f32,
    angular_acceleration: f32,
    radium: f32,
    gravity: f32,
}

impl Pendulum {
    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        Pendulum {
            origin: Vector::new(x, y),
            position: Vector::new(0.0, 0.0),
            angle: 1.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            radium: r,
            gravity: 9.8,
        }
    }

    fn update(&mut self) {
        // Equação do pêndulo para calcular a aceleração angular
        self.angular_acceleration = -1.0 * self.gravity * self.angle.sin() / self.radium;

        // Atualiza velocidade angular
        self.angular_velocity += self.angular_acceleration;

        // Atualiza o ângulo
        self.angle += self.angular_velocity;

        // Calcula coordenadas cartesianas do pêndulo
        self.position.set(
            self.radium * self.angle.sin(),
            self.radium * self.angle.cos(),
        );

        // Soma com a origem
        self.position.add(&self.origin);
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        // Desenha a haste do pêndulo
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::RED,
        );

        // Desenha a massa (círculo)
        graphics.draw_circle(
            (self.position.x, self.position.y),
            20.0,
            Color::BLUE,
        );
    }
}
