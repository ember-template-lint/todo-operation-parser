use std::fmt::Display;

pub mod parser;

// from https://github.com/ember-template-lint/ember-template-lint-todo-utils/blob/6ad4d277c84a74ee0f07341734f7cdbaad21463d/src/types/todos.ts#L70-L81
#[derive(Debug, PartialEq)]
pub struct Position {
    pub line: i32,
    pub column: i32,
}

#[derive(Debug, PartialEq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, PartialEq)]
pub struct TodoData {
    pub engine: String,
    pub rule_id: String,
    pub file_path: String,
    pub range: Range,
    pub source: String,
    pub created_date: i64,
    pub warn_date: i64,
    pub error_date: i64,
}

#[derive(Debug, PartialEq)]
pub enum OperationType {
    Add,
    Remove,
    // TODO: add update
}

#[derive(Debug, PartialEq)]
pub struct TodoOperation {
    operation: OperationType,
    todo: TodoData,
}

impl Display for TodoOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}",
            match self.operation {
                OperationType::Add => "add",
                OperationType::Remove => "remove",
            },
            self.todo.engine,
            self.todo.rule_id,
            self.todo.file_path,
            self.todo.range.start.line,
            self.todo.range.start.column,
            self.todo.range.end.line,
            self.todo.range.end.column,
            self.todo.source,
            self.todo.created_date,
            self.todo.warn_date,
            self.todo.error_date,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn build_simple_operation() -> TodoOperation {
        TodoOperation {
            operation: OperationType::Add,
            todo: TodoData {
                engine: String::from("ember-template-lint"),
                file_path: String::from("some/path/here"),
                rule_id: String::from("bare-strings"),
                range: Range {
                    start: Position { line: 0, column: 0 },
                    end: Position { line: 0, column: 5 },
                },
                source: String::from("hello"),
                created_date: 1000, // TODO: make this more reasonable
                warn_date: 0,
                error_date: 0,
            },
        }
    }

    #[test]
    fn it_can_generate_string_from_todo_data() {
        let todo = build_simple_operation();

        assert_eq!(
            todo.to_string(),
            "add\0ember-template-lint\0bare-strings\0some/path/here\00\00\00\05\0hello\01000\00\00"
        );
    }
}
