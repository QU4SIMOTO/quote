use crate::quote::QuoteItem;

pub struct State<T> {
    quotes: T,
    quote: QuoteItem,
}

impl<T> State<T>
where
    T: Iterator<Item = QuoteItem>,
{
    pub fn new(mut quotes: T) -> Self {
        let quote = quotes.next().unwrap();
        Self { quotes, quote }
    }

    pub fn get_quote(&self) -> &QuoteItem {
        &self.quote
    }

    pub fn next(&mut self) {
        self.quote = self.quotes.next().unwrap();
    }
}
