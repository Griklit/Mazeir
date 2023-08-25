use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Algorithm {
    DepthFirst
}

impl FromStr for Algorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "depth-first" | "DepthFirst" => Ok(Algorithm::DepthFirst),
            _ => Err(format!("{} is not a legal algorithm parameter, try to use depth-first", s))
        }
    }
}