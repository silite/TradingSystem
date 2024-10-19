CREATE TABLE IF NOT EXISTS btcusdt_kline
(
    open_time UInt64,
    open Float64,
    high Float64,
    low Float64,
    close Float64,
    volume Float64,
    close_time UInt64,
    quote_asset_volume Float64,
    number_of_trades UInt64,
    taker_buy_base_asset_volume Float64,
    taker_buy_quote_asset_volume Float64
)
ENGINE = MergeTree()
ORDER BY (open_time);
