use dialoguer::Completion;

pub(super) struct MyCompletion {
    options: Vec<String>,
}

impl Default for MyCompletion {
    fn default() -> Self {
        Self { options: vec![] }
    }
}

impl Completion for MyCompletion {
    /// Simple completion implementation based on substring
    fn get(&self, input: &str) -> Option<String> {
        let matches = self
            .options
            .iter()
            .filter(|option| option.starts_with(input))
            .collect::<Vec<_>>();

        if matches.len() == 1 {
            Some(matches[0].to_string())
        } else {
            None
        }
    }
}
