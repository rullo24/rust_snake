use std::collections::LinkedList;

pub fn linked_list_add_to_back(ref_linked_list: &mut LinkedList<(i8, i8)>, snake_x: i8, snake_y:i8) {
    ref_linked_list.push_back((snake_x, snake_y));
}

// pub fn printing_linked_list(ref_linked_list: &LinkedList<(i8, i8)>){
//     for (snake_x, snake_y) in ref_linked_list {
//         println!("({}, {})", snake_x, snake_y);
//     };
// }