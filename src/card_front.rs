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
    pub gl: Vec<String>,
}

impl CardFront {
    pub fn from_lz(l: &Lz) -> Result<Self, CardErr> {
        let tp = CardType::from_str(&l.get("tp").ok_or(CardErr::NoType)?);
        Ok(match tp {
            CardType::Arc => CardFront {
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
        let a = Args::new()
            .stroke("black")
            .fill("#eeeeee")
            .stroke_width(w / 50.);
        s.rect(0., 0., w, h, a);

        let (wrap, fsize) = if self.name.len() > 18 {
            (14, h / 13.)
        } else {
            (10, h / 11.)
        };
        let nname = wrap_nl(&self.name, wrap);

        let a = Args::new()
            .fill("black")
            .text_anchor("middle")
            .font_weight("bold");
        s.text_lines(&nname, w / 2., h / 8., fsize, fsize, a);
        let ttx = match self.tp {
            CardType::Arc => self.gl.join("\n"),
            _ => wrap_nl(&self.tx, 21),
        };
        let lc = ttx
            .chars()
            .fold(0, |v, c| if c == '\n' { v + 1 } else { v }) as f64;

        let a = Args::new().fill("black").text_anchor("middle");
        s.text_lines(
            &ttx,
            w / 2.,
            (h - (h / 17.) * lc) / 2.,
            h / 18.0,
            h / 17.,
            a,
        );
        let col = self.tp.color();

        let a = Args::new().fill(col);
        s.rect(0., h*0.8,w,h*0.1,a);
        /*
            //Card Type
            barfill := TPColor(bc.tp)
            g.Rect(0, ch*16/20, cw, ch*3/20, fmt.Sprintf("fill:%s;stroke:none;", barfill))
            g.Text(cw/2, ch*9/10, bc.tp, fmt.Sprintf("text-anchor:middle;fill:black;font-family:Arial;font-
        weight:bold;font-size:%dpx", ms/10))

        */
    }
}
