use crate::MergeLine;

pub struct MergeItemDescriptor {
    tier: usize,
    merge_line: MergeLine,
    path: &'static str,
} 

pub fn sweet_merge_line() -> &'static [&'static str] {
    let paths = &[
        "26_chocolate.png",
        "30_chocolatecake.png",
        "34_donut.png",
        "52_gingerbreadman.png",
        "46_fruitcake.png",
        "57_icecream.png",
        "59_jelly.png",
        "61_jam.png",
        "50_giantgummybear.png",
        "90_strawberrycake.png",
        "79_pancakes.png",
    ];
    /*
        let descriptors: Vec<MergeItemDescriptor> = paths.iter().map(|&path| MergeItemDescriptor {
            tier: 1, // Assuming some default tier value
            merge_line: SWEET,
            path: path.to_string(),
        }).collect();
    }
     */
    paths
}

pub fn salty_merge_line() -> &'static [&'static str] {
    &[
        "15_burger.png",
        "32_curry.png",
        "38_friedegg.png",
        "92_sandwich.png",
        "44_frenchfries.png",
        "69_meatball.png",
        "54_hotdog.png",
        "81_pizza.png",
        "85_roastedchicken.png",
        "99_taco.png",
        "95_steak.png"
    ]
}

