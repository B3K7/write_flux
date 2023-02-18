/* infx_w/src/main/rs. */

use influxdb2;
use futures::executor::block_on;
use reqwest::Error;

async fn example( url : &str, org : &str,  token : &str,  bucket : &str, topic : &str, tagunits : &str, tag : &str,  units  : &str, measure  : f64 ) -> Result<(), Box<dyn std::error::Error>> {
    use futures::prelude::*;
    use influxdb2::models::DataPoint;
    use influxdb2::Client;

    let client = Client::new(url, org, token);

    let points = vec![
        DataPoint::builder(topic)
            .tag(tagunits, tag)
            .field(units, measure)
            .build()?
    ];

    client.write(bucket, stream::iter(points)).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error>  {
    let url      = std::env::var("INFLUXDB_URL").unwrap();
    let org      = std::env::var("INFLUXDB_ORG").unwrap();
    let token    = std::env::var("INFLUXDB_TOKEN").unwrap();
    let bucket   = std::env::var("INFLUXDB_BUCKET").unwrap();
    let topic    = "Avail";
    let tagunits = "site";
    let tag      = "01";
    let units    = "avail";
    let measure  = 1.0;
    let _my_result = block_on(example(&url, &org, &token, &bucket, &topic, &tagunits, &tag, &units, measure));

    Ok(())
}
