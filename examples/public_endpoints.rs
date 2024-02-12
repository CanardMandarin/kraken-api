use kraken_api::{
    api::{
        futures::public::analytics::{Analytics, AnalyticsResp},
        query::AsyncQuery,
        spot::public::time::{Time, TimeResp},
    },
    kraken::AsyncKraken,
};

#[tokio::main]
async fn main() {
    let client = AsyncKraken::default();

    let endpoint = Time::builder().build().unwrap();
    let r: TimeResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = Analytics::builder()
        .interval(604800)
        .since(1676556478)
        .build()
        .unwrap();
    let r: AnalyticsResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");
}
