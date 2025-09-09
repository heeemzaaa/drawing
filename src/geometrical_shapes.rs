use rand::prelude::*;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    // fn color() -> Color;
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
    pub fn new(p1:&Point, p2:&Point, p3:&Point) -> Self {
        Self {p1:p1.clone(), p2:p2.clone(), p3:p3.clone()}
    }
}

impl Rectangle {
    pub fn new(p1:&Point, p2:&Point) -> Self {
        Self {p1:p1.clone(), p2:p2.clone()}
    }
}

// impl Circle {
//     pub fn new(center:Point, radius:i32) -> Self {
//         Self {center, radius}
//     }

//     pub fn random(width:i32, height:i32) -> Self {
//         let mut rng = rand::rng();
//         let x = rng.random_range(0..width);
//         let y = rng.random_range(0..height);
//         let center = Point::new(x,y);
//         let radius = rng.random_range(0..width).min(rng.random_range(0..height));
//         Circle::new(center, radius)
//     }
// }

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, Color::white())
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let start_x1 = self.p1.x;
        let start_y1 = self.p1.y;

        let start_x2 = self.p2.x;
        let start_y2 = self.p2.y;

        let dx = start_x2 - start_x1;
        let dy = start_y2 - start_y1;
        
        let mut steps:f64 = dy.abs() as f64;
       
        if dx.abs() > dy.abs() {
            steps = dx.abs() as f64;
        }

        let mut x = start_x1 as f64;
        let mut y = start_y1 as f64;

        let x_inc = dx as f64 / steps;
        let y_inc = dy as f64 / steps;
        
        for _ in 0..steps as usize {
            image.display(x.round() as i32, y.round() as i32, Color::white());
            x += x_inc;
            y += y_inc;
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let p3 = Point::new(self.p1.x, self.p2.y);
        let p4 = Point::new(self.p2.x, self.p1.y);
        
        let line1 = Line::new(self.p1.clone(),p3.clone());
        let line2 = Line::new(p3.clone(),self.p2.clone());        
        let line3 = Line::new(self.p2.clone(),p4.clone());
        let line4 = Line::new(p4.clone(),self.p1.clone());

        Line::draw(&line1, image);
        Line::draw(&line2, image);
        Line::draw(&line3, image);
        Line::draw(&line4, image);
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let line1 = Line::new(self.p1.clone(),self.p2.clone());
        let line2 = Line::new(self.p1.clone(),self.p3.clone());
        let line3 = Line::new(self.p2.clone(),self.p3.clone());
        
        Line::draw(&line1,image);
        Line::draw(&line2,image);
        Line::draw(&line3,image);
    }
}

// impl Drawable for Circle {
//     fn draw(&self, image: &mut Image) {

//     }
// }
