use rand::prelude::*;
use raster::{Color, Image};
use std::f64::consts::PI;

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color() -> Color {
        let mut rng = rand::rng();

        let r: u8 = rng.random_range(0..=255);
        let g: u8 = rng.random_range(0..=255);
        let b: u8 = rng.random_range(0..=255);
        let a: u8 = 255;

        Color { r, g, b, a }
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Circle {
    center: Point,
    radius: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::rng();
        let x = rng.random_range(0..width);
        let y = rng.random_range(0..height);
        Point::new(x, y)
    }
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::rng();
        let x1 = rng.random_range(0..width);
        let y1 = rng.random_range(0..height);
        let p1 = Point::new(x1, y1);

        let x2 = rng.random_range(0..width);
        let y2 = rng.random_range(0..height);
        let p2 = Point::new(x2, y2);

        Line::new(p1, p2)
    }
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Self {
            p1: p1.clone(),
            p2: p2.clone(),
            p3: p3.clone(),
        }
    }
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self {
            p1: p1.clone(),
            p2: p2.clone(),
        }
    }
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::rng();
        let x: i32 = rng.random_range(0..width);
        let y: i32 = rng.random_range(0..height);
        let center: Point = Point::new(x, y);
        let radius: i32 = rng.random_range(0..width).min(rng.random_range(0..height));
        Circle::new(center, radius)
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, Color::white())
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
    let dx: f64 = (self.p2.x - self.p1.x) as f64;
    let dy: f64 = (self.p2.y - self.p1.y) as f64;

    let steps: usize = dx.abs().max(dy.abs()) as usize;

    let x_inc: f64 = dx / steps as f64;
    let y_inc: f64 = dy / steps as f64;

    let mut x: f64 = self.p1.x as f64;
    let mut y: f64 = self.p1.y as f64;

    for _ in 0..=steps {
        image.display(x as i32, y as i32, Color::white());
        x += x_inc;
        y += y_inc;
    }
}
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let p3 = Point::new(self.p1.x, self.p2.y);
        let p4 = Point::new(self.p2.x, self.p1.y);

        Line::new(self.p1.clone(), p3.clone()).draw(image);
        Line::new(p3.clone(), self.p2.clone()).draw(image);
        Line::new(self.p2.clone(), p4.clone()).draw(image);
        Line::new(p4.clone(), self.p1.clone()).draw(image);
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        Line::new(self.p1.clone(), self.p2.clone()).draw(image);
        Line::new(self.p1.clone(), self.p3.clone()).draw(image);
        Line::new(self.p2.clone(), self.p3.clone()).draw(image);
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let mut teta: f64 = 0.0;
        let delta: f64 = 0.001;
        let color = Circle::color();
        while teta < 2.0 * PI {
            let x = self.center.x as f64 + self.radius as f64 * teta.cos();
            let y = self.center.y as f64 + self.radius as f64 * teta.sin();

            image.display(x.round() as i32, y.round() as i32, color.clone());
            teta += delta;
        }
    }
}
