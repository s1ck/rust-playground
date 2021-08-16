use std::io::IntoInnerError;

struct MyStruct {
    a: Vec<i32>,
}

// impl IntoIterator for MyStruct {
//     type Item = i32;

//     type IntoIter = impl Iterator<Item = Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }
