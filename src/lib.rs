use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub name: String,
    pub pattern: String,
    pub replacement: String,
}

pub fn fix_url(input: &str, recipe: Recipe) -> String {
    let regex = Regex::new(&recipe.pattern).unwrap();
    let result = regex.replace(input, &recipe.replacement);
    result.to_string()
}

pub fn find_a_fix(input: &str, recipes: Vec<Recipe>) -> Option<String> {
    recipes.iter().by_ref().find_map(|recipe| {
        let fixed = fix_url(input, recipe.clone());
        if fixed != input {
            Some(fixed)
        } else {
            None
        }
    })
}

pub fn replace_clipboard(recipes: Vec<Recipe>) -> Option<String> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let original = ctx.get_contents().unwrap();
    let fix = find_a_fix(&original, recipes);
    if let Some(f) = fix {
        ctx.set_contents(f).unwrap();
        Some("".to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn should_find_a_fix() {
        let malformed_url = "https://reddit.com/r/unixporn";
        let other_recipe = Recipe {
            name: "jira sucks".to_string(),
            pattern: "/browse/(.*)".to_string(),
            replacement: "/browse/$1?oldIssueView=true".to_string(),
        };
        let recipe = Recipe {
            name: "fix reddit".to_string(),
            pattern: "//reddit.com/".to_string(),
            replacement: "//old.reddit.com/".to_string(),
        };
        let recipes = vec![other_recipe, recipe];
        let result = find_a_fix(&malformed_url, recipes).unwrap();
        assert_eq!(result, "https://old.reddit.com/r/unixporn");
    }
}
