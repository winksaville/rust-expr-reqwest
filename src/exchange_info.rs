use serde::{Deserialize, Serialize};

#[allow(unused)]
use crate::de_string_or_number::{de_string_or_number_to_f64, de_string_or_number_to_u64};

use strum_macros::IntoStaticStr;
#[derive(Debug, Deserialize, Serialize, IntoStaticStr)]
#[serde(tag = "filterType")]
pub enum ExchangeFilters {
    #[serde(rename = "EXCHANGE_MAX_NUM_ORDERS")]
    ExchangeMaxNumOrders {
        #[serde(deserialize_with = "de_string_or_number_to_u64")]
        #[serde(rename = "maxNumOrders")]
        max_num_orders: u64,
    },
    #[serde(rename = "EXCHANGE_MAX_NUM_ALGO_ORDERS")]
    ExchangeMaxAlgoOrders {
        #[serde(deserialize_with = "de_string_or_number_to_u64")]
        #[serde(rename = "maxNumAlgoOrders")]
        max_num_algo_orders: u64,
    },
}

#[derive(Debug, Deserialize, Serialize, IntoStaticStr)]
#[serde(tag = "filterType")]
pub enum SymbolFilters {
    #[serde(rename = "PRICE_FILTER")]
    PriceFilter {
        #[serde(deserialize_with = "de_string_or_number_to_f64")]
        #[serde(rename = "minPrice")]
        min_price: f64,

        #[serde(deserialize_with = "de_string_or_number_to_f64")]
        #[serde(rename = "maxPrice")]
        max_price: f64,

        #[serde(deserialize_with = "de_string_or_number_to_f64")]
        #[serde(rename = "tickSize")]
        tick_size: f64,
    },

    #[serde(rename = "PERCENT_PRICE")]
    PrecentPrice {
        #[serde(deserialize_with = "de_string_or_number_to_f64")]
        #[serde(rename = "multiplierUp")]
        mulitplier_up: f64,

        #[serde(deserialize_with = "de_string_or_number_to_f64")]
        #[serde(rename = "multiplierDown")]
        multiplier_down: f64,

        #[serde(deserialize_with = "de_string_or_number_to_u64")]
        #[serde(rename = "avgPriceMins")]
        avg_price_mins: u64,
    },

    #[serde(rename = "LOT_SIZE")]
    LotSize {
        #[serde(deserialize_with = "de_string_or_number_to_f64")]
        #[serde(rename = "minQty")]
        min_qty: f64,

        #[serde(deserialize_with = "de_string_or_number_to_f64")]
        #[serde(rename = "maxQty")]
        max_qty: f64,

        #[serde(deserialize_with = "de_string_or_number_to_f64")]
        #[serde(rename = "stepSize")]
        step_size: f64,
    },

    #[serde(rename = "MIN_NOTIONAL")]
    MinNotional {
        #[serde(deserialize_with = "de_string_or_number_to_f64")]
        #[serde(rename = "minNotional")]
        min_notional: f64,

        #[serde(rename = "applyToMarket")]
        apply_to_market: bool,

        #[serde(deserialize_with = "de_string_or_number_to_u64")]
        #[serde(rename = "avgPriceMins")]
        avg_price_mins: u64,
    },

    #[serde(rename = "ICEBERG_PARTS")]
    IcebergParts {
        #[serde(deserialize_with = "de_string_or_number_to_u64")]
        limit: u64,
    },

    #[serde(rename = "MARKET_LOT_SIZE")]
    MarketLotSize {
        #[serde(deserialize_with = "de_string_or_number_to_f64")]
        #[serde(rename = "minQty")]
        min_qty: f64,

        #[serde(deserialize_with = "de_string_or_number_to_f64")]
        #[serde(rename = "maxQty")]
        max_qty: f64,

        #[serde(deserialize_with = "de_string_or_number_to_f64")]
        #[serde(rename = "stepSize")]
        step_size: f64,
    },

    #[serde(rename = "MAX_NUM_ORDERS")]
    MaxNumOrders {
        #[serde(deserialize_with = "de_string_or_number_to_u64")]
        #[serde(rename = "maxNumOrders")]
        max_num_orders: u64,
    },

    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrders {
        #[serde(deserialize_with = "de_string_or_number_to_u64")]
        #[serde(rename = "maxNumAlgoOrders")]
        max_num_algo_orders: u64,
    },

    #[serde(rename = "MAX_NUM_ICEBERG_ORDERS")]
    MaxNumIcebergOrders {
        #[serde(deserialize_with = "de_string_or_number_to_u64")]
        #[serde(rename = "maxNumIcebergOrders")]
        max_num_iceberg_orders: u64,
    },

    #[serde(rename = "MAX_POSITION")]
    MaxPosition {
        #[serde(deserialize_with = "de_string_or_number_to_f64")]
        #[serde(rename = "maxPosition")]
        max_position: f64,
    },
}

#[derive(Debug, Deserialize, Serialize, IntoStaticStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    RawRequest,
    RequestWeight,
    Orders,
}

#[derive(Debug, Deserialize, Serialize, IntoStaticStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IntervalType {
    Minute,
    Second,
    Day,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    rate_limit_type: RateLimitType, // Type of rate limit
    interval: IntervalType,         // Type of interval
    interval_num: u64,              // interval_num * interval is a duration
    limit: u64,                     // limit is the maximum rate in the duration.
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    symbol: String,     // +enum BTCUSD?
    base_asset: String, // +enum BTC?
    #[serde(deserialize_with = "de_string_or_number_to_u64")]
    base_asset_precision: u64,
    #[serde(deserialize_with = "de_string_or_number_to_u64")]
    base_commission_precision: u64,
    iceberg_allowed: bool,
    is_margin_trading_allowed: bool,
    is_spot_trading_allowed: bool,
    oco_allowed: bool,
    quote_asset: String, // +enum USD?
    #[serde(deserialize_with = "de_string_or_number_to_u64")]
    quote_asset_precision: u64,
    #[serde(deserialize_with = "de_string_or_number_to_u64")]
    quote_commission_precision: u64,
    quote_order_qty_market_allowed: bool,
    #[serde(deserialize_with = "de_string_or_number_to_u64")]
    quote_precision: u64,
    status: String, // +enum TRADING?
    permissions: Vec<String>,
    order_types: Vec<String>,
    filters: Vec<SymbolFilters>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    #[serde(deserialize_with = "de_string_or_number_to_u64")]
    server_time: u64,
    exchange_filters: Vec<ExchangeFilters>,
    rate_limits: Vec<RateLimit>,
    symbols: Vec<Symbol>,
}

#[cfg(test)]
mod test {
    extern crate test;

    #[allow(unused_imports)]
    use super::*;

    #[allow(unused)]
    const EXCHANGE_INFO_DATA: &str = r#"{
        "serverTime": 1618003698059,
        "exchangeFilters": [
            {
                "filterType": "EXCHANGE_MAX_NUM_ORDERS",
                "maxNumOrders": 123
            },
            {
                "filterType": "EXCHANGE_MAX_NUM_ALGO_ORDERS",
                "maxNumAlgoOrders": "456"
            }
        ],
        "rateLimits": [
            {
                "interval": "MINUTE",
                "intervalNum": 1,
                "limit": 1200,
                "rateLimitType": "RAW_REQUEST"
            },
            {
                "interval": "SECOND",
                "intervalNum": 10,
                "limit": 100,
                "rateLimitType": "REQUEST_WEIGHT"
            },
            {
                "interval": "DAY",
                "intervalNum": 1,
                "limit": 200000,
                "rateLimitType": "ORDERS"
            }
        ],
        "symbols": [
            {
                "symbol": "BTCUSD",
                "baseAsset": "BTC",
                "quoteAsset": "USD",
                "baseAssetPrecision": 8,
                "baseCommissionPrecision": 8,
                "icebergAllowed": true,
                "isMarginTradingAllowed": false,
                "isSpotTradingAllowed": true,
                "ocoAllowed": true,
                "quoteAssetPrecision": 4,
                "quoteCommissionPrecision": 2,
                "quoteOrderQtyMarketAllowed": true,
                "quotePrecision": 4,
                "status": "TRADING",
                "permissions": [
                  "SPOT"
                ],
                "orderTypes": [
                    "LIMIT",
                    "LIMIT_MAKER",
                    "MARKET",
                    "STOP_LOSS_LIMIT",
                    "TAKE_PROFIT_LIMIT"
                ],
                "filters": [
                    {
                        "filterType": "PRICE_FILTER",
                        "maxPrice": "100000.0000",
                        "minPrice": "0.0100",
                        "tickSize": "0.0100"
                    },
                    {
                        "avgPriceMins": 5,
                        "filterType": "PERCENT_PRICE",
                        "multiplierDown": "0.2",
                        "multiplierUp": "5"
                    },
                    {
                        "filterType": "LOT_SIZE",
                        "maxQty": "9000.00000000",
                        "minQty": "0.00000100",
                        "stepSize": "0.00000100"
                    },
                    {
                        "applyToMarket": true,
                        "avgPriceMins": 5,
                        "filterType": "MIN_NOTIONAL",
                        "minNotional": "10.0000"
                    },
                    {
                        "filterType": "ICEBERG_PARTS",
                        "limit": 10
                    },
                    {
                        "filterType": "MARKET_LOT_SIZE",
                        "maxQty": "3200.00000000",
                        "minQty": "0.00000000",
                        "stepSize": "0.00000000"
                    },
                    {
                        "filterType": "MAX_NUM_ORDERS",
                        "maxNumOrders": 200
                    },
                    {
                        "filterType": "MAX_NUM_ALGO_ORDERS",
                        "maxNumAlgoOrders": 5
                    },
                    {
                        "filterType": "MAX_NUM_ICEBERG_ORDERS",
                        "maxNumIcebergOrders": 5
                    },
                    {
                        "filterType": "MAX_POSITION",
                        "maxPosition": 10.0
                    }
                ]
            }
        ]
    }"#;

    #[test]
    fn test_exchange_info() {
        let ei: ExchangeInfo = match serde_json::from_str(EXCHANGE_INFO_DATA) {
            Ok(info) => info,
            Err(e) => panic!("Error processing response: e={}", e),
        };
        println!("ei={:#?}", ei);
        assert_eq!(ei.server_time, 1618003698059u64);

        // To "complex" for texting
        match &ei.exchange_filters[0] {
            ExchangeFilters::ExchangeMaxNumOrders {
                max_num_orders: num,
            } => assert_eq!(*num, 123),
            _ => assert!(false),
        };
        // This is simpler but seem to still need a `match` to access the field
        let ef0 = &ei.exchange_filters[0];
        assert!(matches!(ef0, ExchangeFilters::ExchangeMaxNumOrders { .. }));

        match ei.exchange_filters[1] {
            ExchangeFilters::ExchangeMaxAlgoOrders {
                max_num_algo_orders: num,
            } => assert_eq!(num, 456),
            _ => assert!(false),
        };

        // Using `matches!` is nice for this "homogeneous" structure with typed fields
        let rl0 = &ei.rate_limits[0];
        assert!(matches!(rl0.rate_limit_type, RateLimitType::RawRequest));
        assert!(matches!(rl0.interval, IntervalType::Minute));
        assert_eq!(rl0.interval_num, 1);
        assert_eq!(rl0.limit, 1200);

        let rl1 = &ei.rate_limits[1];
        assert!(matches!(rl1.rate_limit_type, RateLimitType::RequestWeight));
        assert!(matches!(rl1.interval, IntervalType::Second));
        assert_eq!(rl1.interval_num, 10);
        assert_eq!(rl1.limit, 100);

        let rl2 = &ei.rate_limits[2];
        assert!(matches!(rl2.rate_limit_type, RateLimitType::Orders));
        assert!(matches!(rl2.interval, IntervalType::Day));
        assert_eq!(rl2.interval_num, 1);
        assert_eq!(rl2.limit, 200000);

        // Symbols
        let s0 = &ei.symbols[0];
        assert_eq!(s0.symbol, "BTCUSD");
        assert_eq!(s0.base_asset, "BTC");
        assert_eq!(s0.quote_asset, "USD");
        assert_eq!(s0.base_asset_precision, 8);
        assert_eq!(s0.base_commission_precision, 8);
        assert_eq!(s0.iceberg_allowed, true);
        assert_eq!(s0.is_margin_trading_allowed, false);
        assert_eq!(s0.is_spot_trading_allowed, true);
        assert_eq!(s0.oco_allowed, true);
        assert_eq!(s0.quote_asset_precision, 4);
        assert_eq!(s0.quote_commission_precision, 2);
        assert_eq!(s0.quote_order_qty_market_allowed, true);
        assert_eq!(s0.quote_precision, 4);
        assert_eq!(s0.status, "TRADING");
        assert_eq!(s0.permissions, ["SPOT"]);
        assert_eq!(
            s0.order_types,
            [
                "LIMIT",
                "LIMIT_MAKER",
                "MARKET",
                "STOP_LOSS_LIMIT",
                "TAKE_PROFIT_LIMIT",
            ]
        );

        let s0f0 = &s0.filters[0];
        match *s0f0 {
            SymbolFilters::PriceFilter {
                min_price,
                max_price,
                tick_size,
            } => {
                assert_eq!(min_price, 0.01);
                assert_eq!(max_price, 100000.0);
                assert_eq!(tick_size, 0.01);
            }
            _ => assert!(false),
        }
    }
}
