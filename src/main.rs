use clap::{clap_app, crate_version};
use mksvg::page;
use lazy_conf::LzList;
use lazy_conf::{Getable};
use std::str::FromStr;

mod card_type;
use card_type::CardType;

mod card_front;
use card_front::CardFront;


fn main() {
    let clap = clap_app!(
        Shoehorn_Cards =>
            (about:"Build Cards for Shoehorn Circle")
            (version:crate_version!())
            (author:"Matthew Stoodley")
            (@arg out_base:+required "The basename for the output files")
            (@arg files: -f +takes_value ... "File to parse")
    )
    .get_matches();

    let mut cards = Vec::new();
    if let Some(ff) = clap.values_of("files") {
        for f in ff {
            println!("File:{}", f);
            let s = std::fs::read_to_string(f).expect("could not read file");
            let ll = LzList::from_str(&s).expect("not good lz file");
            for (n,lz) in ll.items.iter().enumerate() {
                let cf = CardFront::from_lz(lz).expect(&format!("Problem with lz item {} in {}:\n:{:?}",n,f,lz));
                for _ in 0..lz.get("ext0").and_then(|v|v.parse::<usize>().ok()).unwrap_or(1){
                    cards.push(cf.clone());
                }

            }

        }
    }

    let base_out = clap.value_of("out_base").unwrap();
    let fbase = format!("{}_f_",base_out);
    let bbase = format!("{}_b_",base_out);


    let f_locs = page::pages_a4(fbase,4,4,&cards);


    let cbacks:Vec<CardType>= cards.iter().map(|x|x.tp.clone()).collect();
    let cbacks = page::page_flip(&cbacks,4);
    let b_locs = page::pages_a4(bbase, 4, 4, &cbacks);
    let all_pages = page::interlace(f_locs,b_locs);

    page::unite_as_pdf(all_pages,format!("{}_res.pdf",base_out));

}
