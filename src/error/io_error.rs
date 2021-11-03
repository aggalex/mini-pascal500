use crate::error::{Position, PositionBuilder, Throwable, ERROR};

impl Throwable for std::io::Error {
    fn position(&self, positioner: &PositionBuilder) -> Position {
        positioner.pos(0..0)
    }

    fn title(&self) -> String {
        ERROR.to_string()
    }

    fn description(&self) -> String {
        self.to_string()
    }

    fn notes(&self) -> Vec<String> {
        vec![]
    }
}