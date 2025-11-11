use structs::Lore;

pub fn get_all_availables_lore() -> Vec<Lore> {
    let lores: Vec<Lore> = vec![
        Lore {
            title: "warhammer 40k".to_string(),
        },
        Lore {
            title: "loup garou".to_string(),
        },
        Lore {
            title: "esa magia".to_string(),
        },
    ];

    return lores;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
