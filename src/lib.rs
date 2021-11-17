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

impl From<&str> for OperationType {
    fn from(i: &str) -> Self {
        match i {
            "add" => OperationType::Add,
            "remove" => OperationType::Remove,
            _ => unimplemented!("unknown operation type"),
        }
    }
}
impl From<String> for OperationType {
    fn from(i: String) -> Self {
        match i.as_str() {
            "add" => OperationType::Add,
            "remove" => OperationType::Remove,
            _ => unimplemented!("unknown operation type"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TodoOperation {
    operation: OperationType,
    todo: TodoData,
}

pub trait ToOperation {
    fn to_operation_string(&self) -> String;
}

impl ToOperation for TodoOperation {
    fn to_operation_string(&self) -> String {
        format!(
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

pub fn operation_from_string(s: String) -> TodoOperation {
    let vec: Vec<&str> = s.split('\0').collect();

    TodoOperation {
        operation: match vec[0] {
            "add" => OperationType::Add,
            "remove" => OperationType::Remove,
            _ => panic!("expected valid operation type, got {}", vec[0]),
        },
        todo: TodoData {
            engine: vec[1].to_string(),
            rule_id: vec[2].to_string(),
            file_path: vec[3].to_string(),
            range: Range {
                start: Position {
                    line: i32::from_str_radix(vec[4], 10).expect("valid range.start.line"),
                    column: i32::from_str_radix(vec[5], 10).expect("valid range.start.column"),
                },
                end: Position {
                    line: i32::from_str_radix(vec[6], 10).expect("valid range.end.line"),
                    column: i32::from_str_radix(vec[7], 10).expect("valid range.end.column"),
                },
            },
            source: vec[8].to_string(),
            created_date: i64::from_str_radix(vec[9], 10).expect("valid created_date"),
            warn_date: i64::from_str_radix(vec[10], 10).expect("valid warn_date"),
            error_date: i64::from_str_radix(vec[11], 10).expect("valid error_data"),
        },
    }
}

const GIT_CONFLICT_START: &str = "<<<<<<<";
const GIT_CONFLICT_MIDDLE: &str = "=======";
const GIT_CONFLICT_END: &str = ">>>>>>>";

extern crate nom;
use nom::{
  IResult,
  bytes::complete::{tag, take_while_m_n},
  combinator::map_res,
  sequence::tuple
};

fn operation(input: &str) -> Res<&str, OperationType> {
    context(
        "operation type",
        alt((tag("add"), tag("remove"))),
    )(input)
    .map(|(next_input, res)|
            
}

pub fn parse_operations(s: &str) -> Vec<TodoOperation> {
    let mut operations: Vec<TodoOperation> = Vec::new();

    for line in s.lines() {
        match &line[0..7] {
            GIT_CONFLICT_START => continue,
            GIT_CONFLICT_MIDDLE => continue,
            GIT_CONFLICT_END => continue,
            _ => {
                let operation = operation_from_string(line.to_string());
                operations.push(operation);
            }
        }
    }

    operations
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_simple_operation() -> TodoOperation {
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
    fn it_can_convert_string_operation_to_operation_type() {
        assert_eq!(OperationType::from("add"), OperationType::Add);
        assert_eq!(OperationType::from("remove"), OperationType::Remove);

        assert_eq!(OperationType::from("add".to_string()), OperationType::Add);
        assert_eq!(OperationType::from("remove".to_string()), OperationType::Remove);
    }

    #[test]
    fn it_can_generate_string_from_todo_data() {
        let todo = build_simple_operation();

        assert_eq!(
            todo.to_operation_string(),
            "add\0ember-template-lint\0bare-strings\0some/path/here\00\00\00\05\0hello\01000\00\00"
        );
    }

    #[test]
    fn it_can_read_todo_operation_back_from_string() {
        let todo = build_simple_operation();
        let s = todo.to_operation_string();

        assert_eq!(operation_from_string(s), todo);
    }

    #[test]
    fn it_can_parse_many_operations_from_string() {
        let operations = [
            build_simple_operation(),
            build_simple_operation(),
            build_simple_operation(),
            build_simple_operation(),
        ];
        let s = operations
            .iter()
            .map(|todo| todo.to_operation_string())
            .collect::<Vec<String>>()
            .join("\n");

        assert_eq!(parse_operations(&s), operations);
    }

    #[test]
    fn it_can_handle_git_conflict_markers() {
        let todo_str = build_simple_operation().to_operation_string();
        let theirs_start = "<<<<<<< HEAD";
        let ours_start = "=======";
        let ours_end = ">>>>>>> whatever";

        let conflicted = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            todo_str, theirs_start, todo_str, ours_start, todo_str, ours_end
        );

        assert_eq!(
            parse_operations(&conflicted),
            [
                build_simple_operation(),
                build_simple_operation(),
                build_simple_operation()
            ]
        );
    }
}
