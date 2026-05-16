use nu_engine::command_prelude::*;
#[cfg(feature = "os")]
use nu_protocol::ByteStreamSource;
use nu_protocol::{OutDest, engine::StateWorkingSet};

#[derive(Clone)]
pub struct IgnoreStdout;

impl Command for IgnoreStdout {
    fn name(&self) -> &str {
        "ignore stdout"
    }

    fn description(&self) -> &str {
        "Ignore the stdout of the previous command in the pipeline."
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("ignore stdout")
            .input_output_types(vec![(Type::Any, Type::Nothing)])
            .category(Category::Core)
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["silent", "quiet", "out-null"]
    }

    fn is_const(&self) -> bool {
        true
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        _call: &Call,
        mut input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        if let PipelineData::ByteStream(stream, _) = &mut input {
            #[cfg(feature = "os")]
            if let ByteStreamSource::Child(child) = stream.source_mut() {
                child.ignore_error(true);
                // TODO: This causes SIGPIPE on the child process,
                // but when unused causes the shell to block join child process
                // on `e>|` and `o+e>|` pipes:
                // child.stdout.take();
                // NOTE: See NOTE in ignore_.rs
            }
        }
        input.drain()?;
        Ok(PipelineData::Empty)
    }

    fn run_const(
        &self,
        _working_set: &StateWorkingSet,
        _call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        input.drain()?;
        Ok(PipelineData::Empty)
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Ignore the standard output of an external command.",
                example: "^external arg1 | ignore stdout",
                result: None,
            },
            Example {
                description: "Ignore the output of an echo command.",
                example: "echo 123 | ignore stdout",
                result: Some(Value::nothing(Span::test_data())),
            },
        ]
    }

    fn pipe_redirection(&self) -> (Option<OutDest>, Option<OutDest>) {
        (Some(OutDest::Null), None)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_examples() -> nu_test_support::Result {
        use super::IgnoreStdout;
        nu_test_support::test().examples(IgnoreStdout)
    }
}
