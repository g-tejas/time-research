#![allow(non_snake_case)]

use proto::orderbook_aggregator_client::OrderbookAggregatorClient;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use dioxus::prelude::*;
use rust_decimal_macros::dec;

mod proto {
    tonic::include_proto!("orderbook");
}

#[derive(Clone, Debug, PartialEq)]
pub enum LevelType {
    Bid(i8),
    Ask(i8),
    Spread,
}


#[derive(Clone, Debug, PartialEq)]
pub struct OrderbookData {
    pub level_type: LevelType,
    pub width: String,
    pub color: String,
    pub suffix: String, // Stores the suffix for the level
}

trait SetLevel {
    fn set_level(&mut self, len: Option<u64>, level: &proto::Level);
}

impl SetLevel for OrderbookData {
    fn set_level(&mut self, max_len: Option<u64>, level: &proto::Level) {
        // Width
        let mut width = Decimal::from_f64(level.amount).unwrap() / Decimal::from_f64(max_len.unwrap_or(1) as f64).unwrap() * dec!(100);
        self.width = format!("{}%", width);

        // Suffix
        let mut price = Decimal::from_f64(level.price).unwrap();
        let mut amount = Decimal::from_f64(level.amount).unwrap();
        price.rescale(8);
        amount.rescale(8);
        let msg = format!("{} @ ${},  {}", amount, price, level.exchange);
        self.suffix = msg;
    }
}

#[inline_props]
fn OrderbookLevel(cx: Scope, orderbook_data: OrderbookData) -> Element {
    let OrderbookData { level_type, width, color, suffix } = orderbook_data;
    let prefix = match level_type {
        LevelType::Bid(i) => format!("Bid {}", i),
        LevelType::Ask(i) => format!("Ask {}", i),
        LevelType::Spread => "Spread".to_string(),
    };
    cx.render(rsx! {
            div{
                width: "100%",
                height: "5%",
                flex_direction: "row",
                p {
                    width: "10%",
                    height: "100%",
                    align_items: "center",
                    "{prefix}"
                }
                div {
                    width: "70%",
                    height: "100%",
                    div {
                        width: "{width}",
                        height: "100%",
                        background_color: "{color}",
                    }
                }

                div {
                    width: "20%",
                    height: "100%",

                    p {
                        width: "100%",
                        height: "100%",
                        "{suffix}"
                    }
                }
            }
    })
}

fn spread_percentage(spread: Decimal, best_ask: Option<&proto::Level>) -> Option<Decimal> {
    best_ask
        .map(|l| {
            let mut perc = spread /  Decimal::from_f64(l.price).unwrap() * dec!(100);
            perc.rescale(4);
            perc
        })
}

fn app(cx: Scope) -> Element {
    let bid0 = OrderbookData { level_type: LevelType::Bid(0), width: "0%".to_string(), color: "green".to_string(), suffix: "Bid 0".to_string() };
    let bid1 = OrderbookData { level_type: LevelType::Bid(1), width: "0%".to_string(), color: "green".to_string(), suffix: "Bid 1".to_string() };
    let bid2 = OrderbookData { level_type: LevelType::Bid(2), width: "0%".to_string(), color: "green".to_string(), suffix: "Bid 2".to_string() };
    let bid3 = OrderbookData { level_type: LevelType::Bid(3), width: "0%".to_string(), color: "green".to_string(), suffix: "Bid 3".to_string() };
    let bid4 = OrderbookData { level_type: LevelType::Bid(4), width: "0%".to_string(), color: "green".to_string(), suffix: "Bid 4".to_string() };
    let bid5 = OrderbookData { level_type: LevelType::Bid(5), width: "0%".to_string(), color: "green".to_string(), suffix: "Bid 5".to_string() };
    let bid6 = OrderbookData { level_type: LevelType::Bid(6), width: "0%".to_string(), color: "green".to_string(), suffix: "Bid 6".to_string() };
    let bid7 = OrderbookData { level_type: LevelType::Bid(7), width: "0%".to_string(), color: "green".to_string(), suffix: "Bid 7".to_string() };
    let bid8 = OrderbookData { level_type: LevelType::Bid(8), width: "0%".to_string(), color: "green".to_string(), suffix: "Bid 8".to_string() };
    let bid9 = OrderbookData { level_type: LevelType::Bid(9), width: "0%".to_string(), color: "green".to_string(), suffix: "Bid 9".to_string() };

    let pb_spread = use_ref(cx, || OrderbookData {
        level_type: LevelType::Spread,
        width: "0%".to_string(),
        color: "white".to_string(),
        suffix: "Spread".to_string(),
    });

    let ask0 = OrderbookData { level_type: LevelType::Ask(0), width: "0%".to_string(), color: "red".to_string(), suffix: "Ask 0".to_string() };
    let ask1 = OrderbookData { level_type: LevelType::Ask(1), width: "0%".to_string(), color: "red".to_string(), suffix: "Ask 1".to_string() };
    let ask2 = OrderbookData { level_type: LevelType::Ask(2), width: "0%".to_string(), color: "red".to_string(), suffix: "Ask 2".to_string() };
    let ask3 = OrderbookData { level_type: LevelType::Ask(3), width: "0%".to_string(), color: "red".to_string(), suffix: "Ask 3".to_string() };
    let ask4 = OrderbookData { level_type: LevelType::Ask(4), width: "0%".to_string(), color: "red".to_string(), suffix: "Ask 4".to_string() };
    let ask5 = OrderbookData { level_type: LevelType::Ask(5), width: "0%".to_string(), color: "red".to_string(), suffix: "Ask 5".to_string() };
    let ask6 = OrderbookData { level_type: LevelType::Ask(6), width: "0%".to_string(), color: "red".to_string(), suffix: "Ask 6".to_string() };
    let ask7 = OrderbookData { level_type: LevelType::Ask(7), width: "0%".to_string(), color: "red".to_string(), suffix: "Ask 7".to_string() };
    let ask8 = OrderbookData { level_type: LevelType::Ask(8), width: "0%".to_string(), color: "red".to_string(), suffix: "Ask 8".to_string() };
    let ask9 = OrderbookData { level_type: LevelType::Ask(9), width: "0%".to_string(), color: "red".to_string(), suffix: "Ask 9".to_string() };


    let pb_bids = use_ref(cx, || vec![
        bid0, bid1, bid2, bid3, bid4,
        bid5, bid6, bid7, bid8, bid9
    ]);
    let pb_asks = use_ref(cx, || vec![
        ask0, ask1, ask2, ask3, ask4,
        ask5, ask6, ask7, ask8, ask9
    ]);

    use_coroutine(cx, |rx: UnboundedReceiver<()>| {
        to_owned![pb_bids, pb_spread, pb_asks];
        async move {
            let port: usize = 50051;
            let addr = format!("http://[::1]:{}", port);

            let mut client = OrderbookAggregatorClient::connect(addr).await.unwrap();
            let request = tonic::Request::new(proto::Empty {});

            let mut response = client.book_summary(request).await.unwrap().into_inner();

            // listening to stream
            while let Ok(Some(res)) = response.message().await {
                let proto::Summary{spread, bids, asks} = res;

                // set spread
                let mut spread = Decimal::from_f64(spread).unwrap();
                spread.rescale(8);
                spread_percentage(spread, asks.first())
                    .map(|perc|
                        // pb_spread.set_message(format!("{} ({}%)", spread, perc))
                        pb_spread.write().suffix = format!("{} ({}%)", spread, perc)
                    );

                // Send via tx
                let bid_max_len = bids.iter().map(|l| l.amount as u64).max();
                let ask_max_len = asks.iter().map(|l| l.amount as u64).max();

                // set bids
                bids.iter().rev().enumerate().for_each(|(i, level)|
                    pb_bids.write()[i].set_level(bid_max_len, level)
                );

                // set asks
                asks.iter().enumerate().for_each(|(i, level)|
                    pb_asks.write()[i].set_level(ask_max_len, level)
                );
            }
        }
    });


    cx.render(rsx! {
        div{
            width: "100%",
            height: "100%",
            flex_direction: "column",

            h1 { "Receiving updates from gRPC server" }

            for pb_bid in pb_bids.read().iter().rev() {
                OrderbookLevel {
                    orderbook_data: pb_bid.clone()
                }
            }

            OrderbookLevel {
                orderbook_data: pb_spread.read().clone()
            }

            for pb_ask in pb_asks.read().iter() {
                OrderbookLevel {
                    orderbook_data: pb_ask.clone()
                }
            }

        }
    })
}

fn main() {
    dioxus_tui::launch(app);
}