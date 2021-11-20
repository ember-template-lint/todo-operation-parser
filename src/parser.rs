use crate::{TodoOperation, TodoData, OperationType, Range, Position};

impl<S> From<S> for TodoOperation
where
    S: AsRef<str>,
{
    fn from(s: S) -> Self {
        let vec: Vec<&str> = s.as_ref().split('\0').collect();

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
                        line: vec[4].parse::<i32>().expect("valid range.start.line"),
                        column: vec[5].parse::<i32>().expect("valid range.start.column"),
                    },
                    end: Position {
                        line: vec[6].parse::<i32>().expect("valid range.end.line"),
                        column: vec[7].parse::<i32>().expect("valid range.end.column"),
                    },
                },
                source: vec[8].to_string(),
                created_date: vec[9].parse::<i64>().expect("valid created_date"),
                warn_date: vec[10].parse::<i64>().expect("valid warn_date"),
                error_date: vec[11].parse::<i64>().expect("valid error_data"),
            },
        }
    }
}

const GIT_CONFLICT_START: &str = "<<<<<<<";
const GIT_CONFLICT_MIDDLE: &str = "=======";
const GIT_CONFLICT_END: &str = ">>>>>>>";

pub fn parse_operations(s: &str) -> Vec<TodoOperation> {
    let mut operations: Vec<TodoOperation> = Vec::new();

    for line in s.lines() {
        match &line[0..7] {
            GIT_CONFLICT_START => continue,
            GIT_CONFLICT_MIDDLE => continue,
            GIT_CONFLICT_END => continue,
            _ => {
                operations.push(line.into());
            }
        }
    }

    operations
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::tests::build_simple_operation;
    use crate::TodoOperation;

    #[test]
    fn it_can_read_todo_operation_back_from_string() {
        let todo = build_simple_operation();
        let s = todo.to_string();

        assert_eq!(TodoOperation::from(&s), todo);
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
            .map(|todo| todo.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        assert_eq!(parse_operations(&s), operations);
    }

    #[test]
    fn it_can_handle_git_conflict_markers() {
        let todo_str = build_simple_operation().to_string();
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
