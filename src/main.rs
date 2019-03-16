use mksvg::{page, Args, Card, SvgArg, SvgWrite};

#[derive(Clone, Debug, PartialEq)]
enum CardType {
    Skill,
    Trait,
    Event,
    Scene,
    Scenario,
    Role,
    Arc,
    Whodunnit,
    Other(String),
}

impl CardType {
    fn from_str(s: &str) -> Self {
        use self::CardType::*;
        match s {
            "skill" => Skill,
            "trait" => Trait,
            "event" => Event,
            "scene" => Scene,
            "scenario" => Scenario,
            "role" => Role,
            "arc" => Arc,
            "whodunnit" => Whodunnit,
            v => Other(v.to_string()),
        }
    }

    fn color(&self) -> &'static str {
        match self {
            CardType::Skill => "#99ff99",
            CardType::Trait => "#ff9999",
            CardType::Event => "#ff3333",
            CardType::Scene => "#33ff33",
            CardType::Scenario => "#3333ff",
            CardType::Role => "#9999ff",
            CardType::Arc => "#ffff99",
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
            CardType::Arc => "Arc",
            CardType::Whodunnit => "Whodunnit",
            CardType::Other(s) => &s,
        }
    }
}

impl Card<f64> for CardType {
    fn front<S: SvgWrite>(&self, s: &mut S, w: f64, h: f64) {
        s.rect(0., 0., w, h, Args::new().stroke("none").fill("#eeeeee"));
        s.rect(
            w * 0.2,
            0.,
            w * 0.4,
            h,
            Args::new().stroke("none").fill(self.color()),
        );
        let mut s2 = s.g_rotate(-90., w * 0.5, h * 0.5);
        s2.text(
            self.text(),
            w * 0.5,
            h * 0.5,
            h / 7.,
            Args::new()
                .text_anchor("middle")
                .fill("black")
                .font_family("Arial")
                .font_weight("bold"),
        );
        s2.text(
            "www.storyfeet.com",
            w * 0.5,
            h * 0.75,
            h / 14.0,
            Args::new()
                .text_anchor("middle")
                .fill("#aaaaaa")
                .font_family("Arial")
                .font_weight("bold"),
        );
    }
}

struct CardFront {
    tp: String,
    tx: String,
}

fn main() {
    use self::CardType::*;
    let cards = vec![
        Skill,
        Trait,
        Event,
        Scene,
        Scenario,
        Role,
        Arc,
        Whodunnit,
        Other("OTHER".to_string()),
    ];
    let st = std::io::stdout();
    let w = st.lock();
    page::page_a4(w, 5, 7, &cards);
}
