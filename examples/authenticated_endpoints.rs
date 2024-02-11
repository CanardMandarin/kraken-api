use kraken_api::{api::{query::AsyncQuery, spot::authenticated::account::{balance::{Balance, BalanceResp}, extended_balance::{ExtendedBalance, ExtendedBalanceResp}}}, kraken::AsyncKraken};

#[tokio::main]
async fn main() {

    let endpoint = Balance::builder().build().unwrap();
    let r: BalanceResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = ExtendedBalance::builder().build().unwrap();
    let r: ExtendedBalanceResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");
    
    //     let endpoint = BalanceAvailable::builder()
    //         .symbol("fUSD")
    //         .ty(BalanceType::Funding)
    //         .build()
    //         .unwrap();
    //     ignore(endpoint).query_async(&client).await.unwrap();
    //     // let r: BalanceAvailableResp = endpoint.query_async(&client).await.unwrap();
    //     // println!("{r:#?}");

    //     let endpoint = SubmitFundingOffer::builder()
    //         .ty(FundingOfferType::Limit)
    //         .symbol("fUSD")
    //         .amount(150.)
    //         .rate(0.009)
    //         .period(2)
    //         .hidden(true)
    //         .build()
    //         .unwrap();
    //     ignore(endpoint).query_async(&client).await.unwrap();
    //     // let r: SubmitFundingOfferResp = endpoint.query_async(&client).await.unwrap();
    //     // println!("{r:#?}");

    //     let endpoint = CancelAllFundingOffers::builder()
    //         .currency("USD")
    //         .build()
    //         .unwrap();
    //     ignore(endpoint).query_async(&client).await.unwrap();
    //     // let r: CancelAllFundingOffersResp = endpoint.query_async(&client).await.unwrap();
    //     // println!("{r:#?}");

    //     let endpoint = ActiveFundingOffers::builder()
    //         .symbol("fUSD")
    //         .build()
    //         .unwrap();
    //     ignore(endpoint).query_async(&client).await.unwrap();
    //     // let r: ActiveFundingOffersResp = endpoint.query_async(&client).await.unwrap();
    //     // println!("{r:#?}");

    //     let endpoint = FundingLoans::builder().symbol("fUSD").build().unwrap();
    //     ignore(endpoint).query_async(&client).await.unwrap();
    //     // let r: FundingLoansResp = endpoint.query_async(&client).await.unwrap();
    //     // println!("{r:#?}");

    //     let endpoint = FundingCredits::builder().symbol("fUSD").build().unwrap();
    //     ignore(endpoint).query_async(&client).await.unwrap();
    //     // let r: FundingCreditsResp = endpoint.query_async(&client).await.unwrap();
    //     // println!("{r:#?}");

    //     let endpoint = FundingInfo::builder().symbol("fUSD").build().unwrap();
    //     ignore(endpoint).query_async(&client).await.unwrap();
    //     // let r: FundingInfoResp = endpoint.query_async(&client).await.unwrap();
    //     // println!("{r:#?}");

    //     let endpoint = CancelFundingOffer::builder().id(12345).build().unwrap();
    //     ignore(endpoint).query_async(&client).await.unwrap();
    //     // let r: CancelFundingOfferResp = endpoint.query_async(&client).await.unwrap();
    //     // println!("{r:#?}");

    //     let endpoint = RetrieveOrders::builder().build().unwrap();
    //     ignore(endpoint).query_async(&client).await.unwrap();
    //     // let r: RetrieveOrdersResp = endpoint.query_async(&client).await.unwrap();
    //     // println!("{r:#?}");

    //     let endpoint = RetrieveOrdersBySymbol::builder()
    //         .symbol("tBTCUSD")
    //         .build()
    //         .unwrap();
    //     ignore(endpoint).query_async(&client).await.unwrap();
    //     // let r: RetrieveOrdersBySymbolResp = endpoint.query_async(&client).await.unwrap();
    //     // println!("{r:#?}");

    //     let endpoint = SubmitOrder::builder()
    //         .ty(OrderType::Market)
    //         .symbol("tBTCUSD")
    //         .amount(0.1)
    //         .price(1000.)
    //         .price_aux_limit(1111.)
    //         .flags(vec![OrderFlag::Hidden, OrderFlag::PostOnly])
    //         .build()
    //         .unwrap();
    //     ignore(endpoint).query_async(&client).await.unwrap();
    //     // let r: SubmitOrderResp = endpoint.query_async(&client).await.unwrap();
    //     // println!("{r:#?}");

    //     let endpoint = CancelOrder::builder().id(12345).build().unwrap();
    //     ignore(endpoint).query_async(&client).await.unwrap();
    //     // let r: CancelOrderResp = endpoint.query_async(&client).await.unwrap();
    //     // println!("{r:#?}");

    //     let endpoint = CancelOrders::builder()
    //         .cancel_orders_type(CancelOrdersType::All)
    //         .build()
    //         .unwrap();
    //     ignore(endpoint).query_async(&client).await.unwrap();
    //     // let r: CancelOrdersResp = endpoint.query_async(&client).await.unwrap();
    //     // println!("{r:#?}");

    //     let endpoint = Trades::builder().limit(5).build().unwrap();
    //     ignore(endpoint).query_async(&client).await.unwrap();
    //     // let r: TradesResp = endpoint.query_async(&client).await.unwrap();
    //     // println!("{r:#?}");

    //     let endpoint = OrdersHistory::builder().limit(5).build().unwrap();
    //     ignore(endpoint).query_async(&client).await.unwrap();
    //     // let r: OrdersHistoryResp = endpoint.query_async(&client).await.unwrap();
    //     // println!("{r:#?}");
}
