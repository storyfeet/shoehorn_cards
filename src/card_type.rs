use mksvg::{Card, SvgArg, SvgWrite, Tag, Text};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum CardType {
    Skill,
    Trait,
    Event,
    Scene,
    Scenario,
    Role,
    Goal,
    Whodunnit,
    Other(String),
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CardType::Other(s) => write!(f, "{}", s),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl CardType {
    pub fn from_str(s: &str) -> Self {
        use self::CardType::*;
        match s.to_lowercase().as_ref() {
            "skill" => Skill,
            "trait" => Trait,
            "event" => Event,
            "scene" => Scene,
            "scenario" => Scenario,
            "role" => Role,
            "arc" => Goal,
            "goal" => Goal,
            "whodunnit" => Whodunnit,
            _ => Other(s.to_string()),
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            CardType::Skill => "#cdde87",
            CardType::Trait => "#b7bec8",
            CardType::Event => "#ff3333",
            CardType::Scene => "#33ff33",
            CardType::Scenario => "#3333ff",
            CardType::Role => "#e9afaf",
            CardType::Goal => "#f4ca8c",
            CardType::Whodunnit => "#99ffff",
            CardType::Other(_) => "white",
        }
    }

    fn text(&self) -> &str {
        match self {
            CardType::Skill => "Skill",
            CardType::Trait => "Trait",
            CardType::Event => "Event",
            CardType::Scene => "Scene",
            CardType::Scenario => "Scenario",
            CardType::Role => "Role",
            CardType::Goal => "Goal",
            CardType::Whodunnit => "Whodunnit",
            CardType::Other(s) => &s,
        }
    }
}

impl Card<f64> for CardType {
    fn front<S: SvgWrite>(&self, s: &mut S, w: f64, h: f64) {
        Tag::rect(0., 0., w, h)
            .stroke("none")
            .fill("#eeeeee")
            .write(s);
        Tag::rect(w * 0.2, 0., w * 0.4, h)
            .stroke("none")
            .fill(self.color())
            .write(s);

        let mut s2 = s.g_rotate(-90., w * 0.5, h * 0.5);
        Text::new(self.text(), w * 0.5, h * 0.5, h / 7.)
            .text_anchor("middle")
            .fill("black")
            .font_family("Arial")
            .font_weight("bold")
            .write(&mut s2);

        Text::new("www.storyfeet.com", w * 0.5, h * 0.75, h / 14.)
            .text_anchor("middle")
            .fill("#aaaaaa")
            .font_family("Arial")
            .font_weight("bold")
            .write(&mut s2)
    }
}
