mod completion;
mod history;

use std::fmt::Display;

use anyhow::Result;
use completion::*;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use dialoguer::Select;
use history::*;

pub(crate) fn input<S>(prompt: S) -> Result<String>
where
    S: Into<String>,
{
    println!("Use the Up/Down arrows to scroll through history");
    println!("Use the Right arrow or Tab to complete your command");
    println!();

    let mut history = MyHistory::new();
    let completion = MyCompletion::default();

    Ok(Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .history_with(&mut history)
            .completion_with(&completion)
            // .with_initial_text("".to_string())
            // .validate_with(|input: &String| -> Result<(), &str> {
            //     if input.contains('@') {
            //         Ok(())
            //     } else {
            //         Err("This is not a mail address")
            //     }
            // })
            .interact_text()?)
}

pub(crate) fn select<S>(prompt: S, items: Vec<S>) -> Result<String>
where
    S: Into<String> + Display,
{
    println!("Select one of the following commands:");
    println!();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&items)
        .default(0)
        .interact()?;

    Ok(items[selection].to_string())
}
