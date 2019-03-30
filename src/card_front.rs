use crate::card_type::CardType;
use lazy_conf::{Getable, Lz};
//use mksvg::text::wrap_nl;
use mksvg::{text,  Card, SvgArg, SvgWrite, Tag, Text};

#[derive(Debug, Clone)]
pub enum CardErr {
    NoType,
    NoText,
}

#[derive(Debug, Clone)]
pub struct CardFront {
    pub name: String,
    pub tp: CardType,
    pub tx: String,
    pub gl: Vec<String>,
}

impl CardFront {
    pub fn from_lz(l: &Lz) -> Result<Self, CardErr> {
        let tp = CardType::from_str(&l.get("tp").ok_or(CardErr::NoType)?);
        Ok(match tp {
            CardType::Arc | CardType::Role => CardFront {
                name: l.name.clone(),
                tp,
                tx: l.get("tx").unwrap_or("".to_string()),
                gl: l.get_array("gl", 10),
            },
            _ => CardFront {
                name: l.name.clone(),
                tp,
                tx: mksvg::text::escapes(&l.get("tx").ok_or(CardErr::NoText)?),
                gl: Vec::new(),
            },
        })
    }
}

impl Card<f64> for CardFront {
    fn front<S: SvgWrite>(&self, s: &mut S, w: f64, h: f64) {
        //Background
        Tag::rect(0, 0, w, h)
            .stroke("black")
            .fill("#eeeeee")
            .stroke_width(w / 50.)
            .write(s);

        let (wrap, fsize) = if self.name.len() > 18 {
            (14, h / 13.)
        } else {
            (10, h / 11.)
        };
        let nname = text::wrap(&self.name, wrap);

        Text::lines(&nname, w / 2., h / 8., fsize)
            .fill("black")
            .font_size(fsize)
            .text_anchor("middle")
            .font_weight("bold")
            .write(s);
        let ttx = match self.tp {
            CardType::Arc | CardType::Role => {
                let mut n = 0;
                self.gl
                    .iter()
                    .map(|v| {
                        n += 1;
                        format!("{}. {}", n, v)
                    })
                    .collect()
            }
            _ => mksvg::text::wrap(&self.tx, 21),
        };

        Text::lines(ttx, w / 2., h * 0.55, h / 15.)
            .v_center()
            .fill("black")
            .text_anchor("middle")
            .font_size(h / 18.)
            .write(s);

        let col = self.tp.color();

        Tag::rect(0.,h*0.8,w,h*0.1)
            .fill(col)
            .write(s);

        Text::new(&self.tp.to_string(), w / 2., h * 0.88, h * 0.08)
            .fill("black")
            .font_weight("bold")
            .text_anchor("middle")
            .font_family("Arial")
            .write(s);

        Tag::rect(0., 0., w, h)
            .stroke("black")
            .fill("none")
            .stroke_width(w / 50.)
            .write(s);
    }
}
