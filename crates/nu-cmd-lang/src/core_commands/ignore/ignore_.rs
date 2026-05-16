use nu_engine::command_prelude::*;
#[cfg(feature = "os")]
use nu_protocol::ByteStreamSource;
use nu_protocol::{OutDest, engine::StateWorkingSet};

#[derive(Clone)]
pub struct Ignore;

impl Command for Ignore {
    fn name(&self) -> &str {
        "ignore"
    }

    fn description(&self) -> &str {
        "Ignore all output of the previous command in the pipeline."
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("ignore")
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
                child.stdout.take();
            }
        }
        input.drain()?;
        Ok(PipelineData::empty())
    }

    fn run_const(
        &self,
        _working_set: &StateWorkingSet,
        _call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        input.drain()?;
        Ok(PipelineData::empty())
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Ignore all output of an external command.",
                example: "^external arg1 | ignore",
                result: None,
            },
            Example {
                description: "Ignore the output of an echo command.",
                example: "echo 123 | ignore",
                result: Some(Value::nothing(Span::test_data())),
            },
        ]
    }

    fn pipe_redirection(&self) -> (Option<OutDest>, Option<OutDest>) {
        (Some(OutDest::Null), Some(OutDest::Null))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_examples() -> nu_test_support::Result {
        use super::Ignore;
        nu_test_support::test().examples(Ignore)
    }
}
