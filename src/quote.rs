use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Deserialize)]
pub struct QuoteItem {
    quote: String,
    author: String,
}

impl fmt::Display for QuoteItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} - {}", self.quote, self.author)
    }
}
