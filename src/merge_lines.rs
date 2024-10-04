use crate::MergeLine;

pub struct MergeItemDescriptor {
    tier: usize,
    merge_line: MergeLine,
    pub(crate) path: &'static str,
} 

pub fn sweet_merge_line() -> Vec<MergeItemDescriptor> {
    let sweet_paths = [
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

    generate_mergeline_descriptors(sweet_paths, MergeLine::SWEET)
}

fn generate_mergeline_descriptors(paths: [&'static str; 11], line: MergeLine) -> Vec<MergeItemDescriptor> {
    paths.into_iter().enumerate().map(|(i, path)| {
        MergeItemDescriptor {
            tier: i,
            merge_line: line.clone(),
            path,
        }
    }).collect()
}

pub fn salty_merge_line() -> Vec<MergeItemDescriptor> {
    let salty_paths = [
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
    ];
    generate_mergeline_descriptors(salty_paths, MergeLine::SALTY)
}

