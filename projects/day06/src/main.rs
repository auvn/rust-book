use log::{info};
use std::{io::stdout};

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, msg, record| out.finish(format_args!("[{}] {:?}", record.level(), msg)))
        .chain(stdout())
        .apply()?;
    Ok(())
}

#[derive(Debug)]
struct TokenConfig {
    symbol: String,
    precision: Option<u32>,
}

impl TokenConfig {
    fn round(&self, a: f64) -> f64 {
        let div = match self.precision {
            Some(p) => (p as f64) * 10.0,
            _ => 1.0,
        };
        (a * div).round() / div
    }

    fn format(&self, a: f64) -> String {
        let symbol = &self.symbol;
        let rounded = self.round(a);

        format!("{rounded} {symbol}").to_string()
    }
}

#[derive(Debug)]
enum Token {
    Sol(TokenConfig),
    Eth(TokenConfig),
    Pol(TokenConfig),
    Matic(TokenConfig),
}

impl Token {
    fn amount(self, a: f64) -> TokenAmount {
        TokenAmount {
            token: self,
            amount: a,
        }
    }
}

struct TokenAmount {
    token: Token,
    amount: f64,
}

fn print_price(t: &TokenAmount) {
    let value = match &t.token {
        Token::Sol(c) => format!("Solana: {}", c.format(t.amount)),
        Token::Eth(c) => format!("Ethereum: {}", c.format(t.amount)),
        Token::Pol(c) => format!("Polygon: {}", c.format(t.amount)),
        Token::Matic(c) => format!("Polygon: {}", c.format(t.amount)),
    };

    info!("{value}");
}

fn main() {
    setup_logger().expect("Failed to configure logger");

    let sol = Token::Sol(TokenConfig {
        symbol: "SOL".to_string(),
        precision: Some(5),
    });
    let eth = Token::Eth(TokenConfig {
        symbol: "ETH".to_string(),
        precision: Some(7),
    });
    let pol = Token::Pol(TokenConfig {
        symbol: "POL".to_string(),
        precision: None,
    });
    let matic = Token::Matic(TokenConfig {
        symbol: "MATIC".to_string(),
        precision: Some(22),
    });

    let sol_amount = sol.amount(10.);
    let eth_amount = eth.amount(345.45);
    let pol_amount = pol.amount(1024.432);
    let matic_amount = matic.amount(34.123_124_324_324_13);

    print_price(&sol_amount);
    print_price(&eth_amount);
    print_price(&pol_amount);
    print_price(&matic_amount);
}
