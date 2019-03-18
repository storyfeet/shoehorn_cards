use crate::card_type::CardType;
use lazy_conf::{Getable, Lz};
use mksvg::text::wrap_nl;
use mksvg::{Args, Card, SvgArg, SvgWrite};

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
}

impl CardFront {
    pub fn from_lz(l: &Lz) -> Result<Self, CardErr> {
        let tp = CardType::from_str(&l.get("tp").ok_or(CardErr::NoType)?);
        Ok(match tp {
            CardType::Arc => CardFront {
                name: l.name.clone(),
                tp,
                tx: l.get("tx").unwrap_or("".to_string()),
            },
            _ => CardFront {
                name: l.name.clone(),
                tp,
                tx: l.get("tx").ok_or(CardErr::NoText)?,
            },
        })
    }
}

impl Card<f64> for CardFront {
    fn front<S: SvgWrite>(&self, s: &mut S, w: f64, h: f64) {
        //Background
        let a = Args::new()
            .stroke("black")
            .fill("#eeeeee")
            .stroke_width(w / 50.);
        s.rect(0., 0., w, h, a);

        let nname = wrap_nl(&self.name, 10);

        let a = Args::new().fill("black").text_anchor("middle");
        s.text_lines(&nname, w / 2., h / 8., h / 11., h / 11., a);
        /*
            //Title
            g.Textlines(cw/2, ch*2/16, nname, ch/11, ch/11, "black", "middle", "stroke:none;font-weight:
        bold;font-family:Arial")

            // Text
            ttx := msvg.Wrap(msvg.NL(bc.tx), 22)
            msvg.Lines(g, cw/2, ch*5/16, ch/17, ttx, fmt.Sprintf("stroke:none;fill:black;text-anchor:middle;
        font-family:Arial;font-size:%dpx", ch/17))
            //g.Textlines(cw/2, ch*5/16, ttx, ch/17, ch/17, "black", "middle", "stroke:none;")

            //Card Type
            barfill := TPColor(bc.tp)
            g.Rect(0, ch*16/20, cw, ch*3/20, fmt.Sprintf("fill:%s;stroke:none;", barfill))
            g.Text(cw/2, ch*9/10, bc.tp, fmt.Sprintf("text-anchor:middle;fill:black;font-family:Arial;font-
        weight:bold;font-size:%dpx", ms/10))

        */
    }
}
