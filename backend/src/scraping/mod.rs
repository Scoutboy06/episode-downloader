mod request;

use scraper::{element_ref::Text, ElementRef, Html, Selector};

pub use self::request::*;

pub trait SelectOne {
    fn select_one(&self, selector: &'static str) -> Option<ElementRef>;
}

impl<'a> SelectOne for ElementRef<'a> {
    fn select_one(&self, selector: &'static str) -> Option<ElementRef> {
        let s = Selector::parse(&selector).unwrap();
        self.select(&s).next()
    }
}

impl SelectOne for Html {
    fn select_one(&self, selector: &'static str) -> Option<ElementRef> {
        let s = Selector::parse(&selector).unwrap();
        self.select(&s).next()
    }
}

pub trait CollectTrim {
    fn collect_trimmed(&mut self) -> String;
}

impl<'a> CollectTrim for Text<'a> {
    fn collect_trimmed(&mut self) -> String {
        let mut out = String::new();

        for text in self.into_iter() {
            out.push_str(text.trim());
        }

        out
    }
}
