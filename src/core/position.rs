use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub struct Position {
    pub row: u8,
    pub column: u8,
}

impl Position {
    pub fn new(row: u8, column: u8) -> Position {
        Position { row, column }
    }

    pub fn move_towards(&mut self, new_position: &Position) {
        if self.row < new_position.row {
            self.row += 1;
        } else if self.row > new_position.row {
            self.row -= 1;
        }

        if self.column < new_position.column {
            self.column += 1;
        } else if self.column > new_position.column {
            self.column -= 1;
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = String::from("Position { row: ");
        result.push_str(format!("{}", self.row).as_str());
        result.push_str(", column: ");
        result.push_str(format!("{}", self.column).as_str());
        result.push_str(" }");

        write!(f, "{}", result)
    }
}
