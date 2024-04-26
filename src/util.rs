use clap::builder::styling::AnsiColor;
use clap::builder::Styles;
use clap::ValueEnum;
use log::trace;
use std::error::Error;
use std::{env, fmt};
use toml::Table;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Language {
    Rust = 1,
    Python = 2,
    C = 3,
    Lua = 4,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Rust => write!(f, "Rust"),
            Self::Python => write!(f, "Python"),
            Self::C => write!(f, "C"),
            Self::Lua => write!(f, "Lua"),
        }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Difficulty {
    /// Perfect recall
    VeryEasy,
    /// Correct response after some hesitation
    Easy,
    /// Correct response, but took significant effort to recall
    Medium,
    /// Incorrect answer, but upon seeing correct answer or incorrect test, answer was easy to
    /// recall
    Hard,
    /// Incorrect answer, but upon seeing correct answer or incorrect test, answer seemed familiar
    VeryHard,
    /// Complete failure; no clue
    Again,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Again => write!(f, "Again"),
            Self::VeryHard => write!(f, "Very Hard"),
            Self::Hard => write!(f, "Hard"),
            Self::Medium => write!(f, "Medium"),
            Self::Easy => write!(f, "Easy"),
            Self::VeryEasy => write!(f, "Very Easy"),
        }
    }
}

pub fn get_style() -> clap::builder::Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default())
}

/// currently, uses env variable KATA_CFG to find user config file location
pub fn parse_config() -> Result<toml::Table, Box<dyn Error>> {
    let cfg_loc =
        env::var("KATA_CFG").expect("KATA_CFG environment variable should be set by the user");
    let value = std::fs::read_to_string(cfg_loc)?
        .parse::<Table>()
        .expect("file should be TOML parsible");

    Ok(value)
}

pub fn sm2_algo(
    user_grade: Difficulty,
    repetition_number: i32,
    easiness_factor: f32,
    interval: i32,
) -> (i32, f32, i32) {
    trace!("SM2 algo");
    let mut new_rep_num = repetition_number;
    let new_interval: i32;

    match user_grade {
        Difficulty::VeryEasy | Difficulty::Easy | Difficulty::Medium => {
            match repetition_number {
                0 => new_interval = 1,
                1 => new_interval = 6,
                _ => new_interval = (easiness_factor * (interval as f32)).round() as i32,
            };
            new_rep_num += 1;
        }
        _ => {
            new_interval = 1;
            new_rep_num = 0;
        }
    }
    let mut new_easiness_factor = easiness_factor
        + (0.1
            - (5.0 - user_grade as isize as f32)
                * (0.08 + (5.0 - user_grade as isize as f32) * 0.02));
    if new_easiness_factor < 1.3 {
        new_easiness_factor = 1.3
    }

    (new_rep_num, new_easiness_factor, new_interval)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sm2_rep() {
        let result = sm2_algo(Difficulty::Medium, 0, 2.5, 1);
        assert_eq!(result.0, 1);
    }

    #[test]
    fn test_sm2_ef() {
        let result = sm2_algo(Difficulty::Medium, 0, 2.5, 1);
        println!("{:?}", &result.1);
        assert_eq!(result.1, 2.18);
    }

    #[test]
    fn test_sm2_interval() {
        let result = sm2_algo(Difficulty::Medium, 0, 2.5, 1);
        assert_eq!(result.2, 1);
    }
}
