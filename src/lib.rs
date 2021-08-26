use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Recipe {
    name: String,
    pattern: String,
    replacement: String,
}

pub fn fix_url(input: &str, recipe: Recipe) -> String {
    let regex = Regex::new(&recipe.pattern).unwrap();
    let result = regex.replace(input, &recipe.replacement);
    result.to_string()
}

pub fn example() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    println!("{:?}", ctx.get_contents());
    ctx.set_contents("some string".to_owned()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_steal_clipboard() {
        example();
        assert_eq!(1, 1);
    }

    #[test]
    fn should_fix_url() {
        let malformed_url = "https://reddit.com/r/unixporn";
        let recipe = Recipe {
            name: "fix reddit".to_string(),
            pattern: "//reddit.com/".to_string(),
            replacement: "//old.reddit.com/".to_string(),
        };
        let result = fix_url(&malformed_url, recipe);
        assert_eq!(result, "https://old.reddit.com/r/unixporn");
    }
}
