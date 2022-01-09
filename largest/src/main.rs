// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest = list[0];
// 
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
// 
//     largest
// }
// 
// fn largest_char(list: &[char]) -> char {
//     let mut largest = list[0];
// 
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
// 
//     largest
// }
// 
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
// 
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
// 
//     largest
// }

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    let p1 = Point { x: 5, y: 10.6 };
    println!("{:?}", p1);
    let p2 = Point { x: "Hello", y: "Rust" };
    println!("{:?}", p2);
    let p3 = p1.mixup(p2);
    println!("{:?}", p3);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
