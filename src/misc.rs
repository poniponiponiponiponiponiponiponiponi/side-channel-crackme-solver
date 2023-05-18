use crate::command::{PreparedCommand, InputPreparer};
use crate::command;

pub fn find_length(
        max_size: usize,
        input_preparer: &InputPreparer,
        command: &PreparedCommand) -> usize {
    let mut prefix = String::new();
    let mut execution_times = Vec::new();
    for _ in 0..=max_size {
        let val = command::parse_output(&command.run(&input_preparer.prepare(&prefix)));
        execution_times.push((val, prefix.len()));
        prefix.push('A');
    }

    let mut delta_vector: Vec<_> = execution_times.windows(2).map(|e| {
        (e[1].0-e[0].0, e[1].1)
    }).collect();
    delta_vector.sort();

    return delta_vector.last().unwrap().1;
}
