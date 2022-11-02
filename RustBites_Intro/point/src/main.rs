// Generic Point Demo

#![allow(dead_code)]
use std::fmt::*;

#[derive(Debug, Default, Clone)]
pub struct Point<T>
  where T:Debug + Default + Clone {
    coordinates : Vec<T>, 
    name : String
}

use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Point<T> 
  where T:Debug + Default + Clone {
  type Output = T;

  fn index(&self, index: usize) -> &Self::Output {
    &self.coordinates[index]
  }
}

impl<T> IndexMut<usize> for Point<T> 
  where T:Debug + Default + Clone {

  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.coordinates[index]
  }
}

impl<T>  Point<T>
  where T:Debug + Default + Clone {
    fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    fn get_name<'a>(&'a mut self) -> & 'a str {
        &self.name
    }

    fn get_coords_string(&self) -> String {
        let mut coords:String = String::from("");
        for coor in self.coordinates.iter() {
            coords.push_str(&format!("{:?} ", &coor));
        }
        coords
    }

    fn show_point(&self) {
        print!("\n  {} = {:?}", &self.name, &self);
    }
}

fn main() {
  let p1 = Point::<i32> { 
    coordinates:vec![1,2,3], 
    name:String::from("p1")
  };
  print!("\n  Hello point p1:\n  {:?}\n", p1);

  let p2 = p1.clone();
  print!("\n  Hello cloned point p2:\n  {:?}\n", p2);

  let mut p3 = Point::<f64> {
    coordinates: vec![0.5, -1.5, 2.0],
    name:String::from("p3")
  };
  p3.coordinates[2] += 1.5;
  print!("\n  Hello float point p3:\n  {:?}", p3);
  print!("\n  p3[1] = {:?}", p3[1]);

  let mut p4 = p3.clone();
  p4.set_name("p4");
  p4[2] = - p4[2];
  
  print!("\n\n  p4 = {:?}", p4);
  p4.show_point();
  print!("\n  p4 name is {:?}", p4.get_name());
  print!("\n  p4 coordinates are: {:?}\n\n", p4.get_coords_string());
}

// ref: https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings

