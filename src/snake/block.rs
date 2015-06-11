use location::Location;

#[derive(Debug)]
pub struct Block {
    pub location: Location,
    pub blockType: BlockType
}

#[derive(Debug)]
pub enum BlockType {
    Wall,
    Empty,
    SnakeHead,
    SnakeBody,
    Apple
}

