use location::Location;

#[derive(Debug)]
pub struct Block {
    pub location: Location,
    pub block_type: BlockType
}

#[derive(Debug)]
pub enum BlockType {
    Wall,
    Empty,
    SnakeHead,
    SnakeBody,
    Apple
}

