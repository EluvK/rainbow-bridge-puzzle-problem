use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum LineType {
    Arch,
    Cross,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum SingleColor {
    Red,
    Blue,
    Green,
    Yellow,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Color {
    RedBlue,
    RedGreen,
    RedYellow,
    BlueGreen,
    BlueYellow,
    GreenYellow,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Block {
    line_type: LineType,
    color: Color,

    // for Cross, the default 0 is left-right color is first color,
    // for Arch, the default 0 is left-top color is first color,
    // then rotate the angle by clockwise
    angle: i32, // 0, 90, 180, 270
}

impl Block {
    fn is_some_type(&self, other_block: &Block) -> bool {
        self.line_type == other_block.line_type && self.color == other_block.color
    }
}

fn rectangle_map(a: usize, b: usize) -> Vec<Vec<Option<Block>>> {
    let mut map = Vec::new();
    for _ in 0..a {
        let mut row = Vec::new();
        for _ in 0..b {
            row.push(None);
        }
        map.push(row);
    }
    map
}
fn standard_blocks() -> HashMap<Block, i32> {
    let mut blocks = HashMap::new();
    // insert each type of block for twice
    static LINE_TYPES: [LineType; 2] = [LineType::Arch, LineType::Cross];
    static COLORS: [Color; 6] = [
        Color::RedBlue,
        Color::RedGreen,
        Color::RedYellow,
        Color::BlueGreen,
        Color::BlueYellow,
        Color::GreenYellow,
    ];
    LINE_TYPES.iter().for_each(|line_type| {
        COLORS.iter().for_each(|color| {
            blocks.insert(
                Block {
                    line_type: line_type.clone(),
                    color: color.clone(),
                    angle: 0, // whatever here
                },
                2,
            );
        });
    });
    blocks
}

fn get_single_color(color: &Color, is_first_color: bool) -> SingleColor {
    match (color, is_first_color) {
        (Color::RedBlue, true) => SingleColor::Red,
        (Color::RedBlue, false) => SingleColor::Blue,
        (Color::RedGreen, true) => SingleColor::Red,
        (Color::RedGreen, false) => SingleColor::Green,
        (Color::RedYellow, true) => SingleColor::Red,
        (Color::RedYellow, false) => SingleColor::Yellow,
        (Color::BlueGreen, true) => SingleColor::Blue,
        (Color::BlueGreen, false) => SingleColor::Green,
        (Color::BlueYellow, true) => SingleColor::Blue,
        (Color::BlueYellow, false) => SingleColor::Yellow,
        (Color::GreenYellow, true) => SingleColor::Green,
        (Color::GreenYellow, false) => SingleColor::Yellow,
    }
}

fn get_top_color(block: &Block, angle: i32) -> SingleColor {
    let is_first_color = match (&block.line_type, angle) {
        (LineType::Arch, 0) => true,
        (LineType::Arch, 90) => true,
        (LineType::Arch, 180) => false,
        (LineType::Arch, 270) => false,
        (LineType::Cross, 0) => false,
        (LineType::Cross, 90) => true,
        _ => todo!(),
    };
    get_single_color(&block.color, is_first_color)
}

fn get_bottom_color(block: &Block, angle: i32) -> SingleColor {
    let is_first_color = match (&block.line_type, angle) {
        (LineType::Arch, 0) => false,
        (LineType::Arch, 90) => false,
        (LineType::Arch, 180) => true,
        (LineType::Arch, 270) => true,
        (LineType::Cross, 0) => false,
        (LineType::Cross, 90) => true,
        _ => todo!(),
    };
    get_single_color(&block.color, is_first_color)
}

fn get_left_color(block: &Block, angle: i32) -> SingleColor {
    let is_first_color = match (&block.line_type, angle) {
        (LineType::Arch, 0) => true,
        (LineType::Arch, 90) => false,
        (LineType::Arch, 180) => false,
        (LineType::Arch, 270) => true,
        (LineType::Cross, 0) => true,
        (LineType::Cross, 90) => false,
        _ => todo!(),
    };
    get_single_color(&block.color, is_first_color)
}

fn get_right_color(block: &Block, angle: i32) -> SingleColor {
    let is_first_color = match (&block.line_type, angle) {
        (LineType::Arch, 0) => false,
        (LineType::Arch, 90) => true,
        (LineType::Arch, 180) => true,
        (LineType::Arch, 270) => false,
        (LineType::Cross, 0) => true,
        (LineType::Cross, 90) => false,
        _ => todo!(),
    };
    get_single_color(&block.color, is_first_color)
}

/// check if a block can be put in position (x, y)
fn is_valid_block(
    map: &[Vec<Option<Block>>],
    block: &Block,
    x: usize,
    y: usize,
    angle: i32,
) -> bool {
    if x != 0 {
        // check if this block ok with the block above(x-1,y)
        let above_block = map[x - 1][y].clone().unwrap();
        if get_top_color(block, angle) != get_bottom_color(&above_block, above_block.angle) {
            return false;
        }
        // a self-defined rule to make result more beautiful
        if above_block.is_some_type(block) {
            return false;
        }
    }
    if y != 0 {
        // check if this block ok with the block left(x,y-1)
        let left_block = map[x][y - 1].clone().unwrap();
        if get_left_color(block, angle) != get_right_color(&left_block, left_block.angle) {
            return false;
        }
        // a self-defined rule to make result more beautiful
        if left_block.is_some_type(block) {
            return false;
        }
    }

    true
}

fn try_next_block(
    map: &mut Vec<Vec<Option<Block>>>,
    blocks: &mut HashMap<Block, i32>,
    x: usize,
    y: usize,
) -> bool {
    if x != map.len() - 1 {
        if y != map[0].len() - 1 {
            dfs_put_block(map, blocks, x, y + 1)
        } else {
            dfs_put_block(map, blocks, x + 1, 0)
        }
    } else if y != map[0].len() - 1 {
        dfs_put_block(map, blocks, x, y + 1)
    } else {
        println!("{:?}", map);
        true
    }
}

/// try to put a block on the map[x][y]
fn dfs_put_block(
    map: &mut Vec<Vec<Option<Block>>>,
    blocks: &mut HashMap<Block, i32>,
    x: usize,
    y: usize,
) -> bool {
    println!("dfs_put_block({}, {})", x, y);
    // fetch a block from blocks
    let mut current_left_blocks = blocks.clone();

    for (some_block, left_num) in blocks.iter() {
        // test each possible block type
        if *left_num <= 0 {
            continue;
        }
        let mut chosen_block = some_block.clone();
        println!("chosen_block: {:?}", chosen_block);

        let possible_angle = match chosen_block.line_type {
            LineType::Arch => {
                // for arch, there is for possible angle. 0, 90, 180, 270
                vec![0, 90, 180, 270]
            }
            LineType::Cross => {
                // for Cross, there is only two possible angle. 0, 90
                vec![0, 90]
            }
        };
        let mut solved = false;
        // for each possible angle, try to put the block on the map and dfs
        possible_angle.iter().for_each(|angle| {
            if solved {
                return;
            }
            if is_valid_block(map, &chosen_block, x, y, *angle) {
                chosen_block.angle = *angle;
                *current_left_blocks
                    .iter_mut()
                    .find(|(b, _)| b.is_some_type(&chosen_block))
                    .unwrap()
                    .1 -= 1;
                map[x][y] = Some(chosen_block.clone());
                if try_next_block(map, &mut current_left_blocks, x, y) {
                    solved = true;
                }
                *current_left_blocks
                    .iter_mut()
                    .find(|(b, _)| b.is_some_type(&chosen_block))
                    .unwrap()
                    .1 += 1;
            }
        });
        if solved {
            return true;
        }
    }
    false
}

fn main() {
    let mut blocks = standard_blocks();
    println!("{:?}", blocks);

    let mut map = rectangle_map(6, 4);
    let r = dfs_put_block(&mut map, &mut blocks, 0, 0);
    println!("result: {}", r);
}
