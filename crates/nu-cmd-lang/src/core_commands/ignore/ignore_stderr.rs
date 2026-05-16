use nu_engine::command_prelude::*;
#[cfg(feature = "os")]
use nu_protocol::ByteStreamSource;
use nu_protocol::OutDest;

#[derive(Clone)]
pub struct IgnoreStderr;

impl Command for IgnoreStderr {
    fn name(&self) -> &str {
        "ignore stderr"
    }

    fn description(&self) -> &str {
        "Ignore the stderr of the previous command in the pipeline."
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("ignore stderr")
            .input_output_types(vec![(Type::Any, Type::Any)])
            .category(Category::Core)
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["silent", "quiet", "err-null"]
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
        if let PipelineData::ByteStream(stream, ..) = &mut input {
            #[cfg(feature = "os")]
            if let ByteStreamSource::Child(child) = stream.source_mut() {
                child.stderr.take();
            }
        }
        Ok(input)
    }

    fn run_const(
        &self,
        _working_set: &StateWorkingSet,
        _call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        Ok(input)
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Ignore the error output of an external command.",
                example: "^external arg1 | ignore stderr",
                result: None,
            },
            Example {
                description: "Do nothing to the output of a builtin echo command.",
                example: "echo 123 | ignore stderr",
                result: Some(Value::int(123, Span::test_data())),
            },
        ]
    }

    fn pipe_redirection(&self) -> (Option<OutDest>, Option<OutDest>) {
        (None, Some(OutDest::Null))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_examples() -> nu_test_support::Result {
        use super::IgnoreStderr;
        nu_test_support::test().examples(IgnoreStderr)
    }
}
