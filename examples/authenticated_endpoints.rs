use kraken_api::{
    api::{
        futures::authenticated::{
            accounts::{Accounts, AccountsResp},
            historical_fuding_rates::{HistoricalFundingRates, HistoricalFundingRatesResp},
            instruments::{Instruments, InstrumentsResp},
            open_positions::{OpenPositions, OpenPositionsResp},
            orderbook::{OrderBook, OrderBookResp}, withdrawal::{Withdrawal, WithdrawalResp},
        },
        query::AsyncQuery,
        spot::authenticated::{account::{
            balance::{Balance, BalanceResp},
            extended_balance::{ExtendedBalance, ExtendedBalanceResp},
        }, wallet_transfer::{WalletTransfer, WalletTransferResp}},
    },
    kraken::AsyncKraken,
};

#[tokio::main]
async fn main() {
    let spot_client = AsyncKraken::new_auth("YOUR_API_KEY", "YOUR_SECRET_KEY");
    let futures_client = AsyncKraken::new_auth("YOUR_API_KEY", "YOUR_SECRET_KEY");

    let endpoint = Balance::builder().build().unwrap();
    let r: BalanceResp = endpoint.query_async(&spot_client).await.unwrap();
    println!("{r:#?}");

    let endpoint = ExtendedBalance::builder().build().unwrap();
    let r: ExtendedBalanceResp = endpoint.query_async(&spot_client).await.unwrap();
    println!("{r:#?}");

    let endpoint = Accounts::builder().build().unwrap();
    let r: AccountsResp = endpoint.query_async(&futures_client).await.unwrap();
    println!("{r:#?}");

    let endpoint = OpenPositions::builder().build().unwrap();
    let r: OpenPositionsResp = endpoint.query_async(&futures_client).await.unwrap();
    println!("{r:#?}");

    let endpoint = HistoricalFundingRates::builder()
        .symbol("PF_DYMUSD".to_string())
        .build()
        .unwrap();
    let r: HistoricalFundingRatesResp = endpoint.query_async(&futures_client).await.unwrap();
    println!("{r:#?}");

    let endpoint = OrderBook::builder()
        .symbol("PF_DYMUSD".to_string())
        .build()
        .unwrap();
    let r: OrderBookResp = endpoint.query_async(&futures_client).await.unwrap();
    println!("{r:#?}");

    let endpoint = Instruments::builder().build().unwrap();
    let r: InstrumentsResp = endpoint.query_async(&futures_client).await.unwrap();
    println!("{r:#?}");

    let endpoint = Withdrawal::builder()
        .amount("1.0")
        .currency("doge")
        .source_wallet("flex")
        .build()
        .unwrap();
    let r: WithdrawalResp = endpoint.query_async(&futures_client).await.unwrap();
    println!("{r:#?}");

    let endpoint = WalletTransfer::builder().amount(1.0).asset("doge").build().unwrap();
    let r: WalletTransferResp = endpoint.query_async(&spot_client).await.unwrap();
    println!("{r:#?}");
}
