/* infx_w/src/main/rs. */

use influxdb2;
use futures::executor::block_on;
use reqwest::Error;
use std::time::SystemTime;
use std::fs;
use std::convert::TryFrom;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct FlxStruct {
    url    : String,
    org    : String,
    token  : String,
    bucket : String,
}

async fn example( url : &str, org : &str,  token : &str,  bucket : &str, topic : &str, tagunits : &str, tag : &str,  units  : &str, measure  : f64, dur : &std::time::Duration ) -> Result<(), Box<dyn std::error::Error>> {
    use futures::prelude::*;
    use influxdb2::models::DataPoint;
    use influxdb2::Client;

    let t_units  = "sec";
    let u_secs :u64 = dur.as_secs();
    let secs   :i64   = match i64::try_from(u_secs) {
        Ok(n) => n,
        Err(_) => panic!("seconds outside LOQ."),
    };

    const BILLION :u128  = 1_000_000_000u128;
    let nu_secs : u128 = match u128::try_from(u_secs) {
        Ok(n) => n*BILLION ,
        Err(_) => panic!("nu_sec outside LOQ."),
    };

    let n_units  = "nsec";
    let u_nanos :u128 = dur.as_nanos() ;
    let nanos   :i64  = match i64::try_from(u_nanos - nu_secs ) {
        Ok(n) => n,
        Err(_) => panic!("nanoseconds outside LOQ."),
    };

    let client = Client::new(url, org, token);

    let points = vec![
        DataPoint::builder(topic)
            .tag(tagunits, tag)
            .field(units, measure)
            .field(t_units, secs )
            .field(n_units, nanos )
            .build()?
    ];

    client.write(bucket, stream::iter(points)).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error>  {
    let input_path = "./nfx.db";

    let nfx_db = {
        let nfx_db = fs::read_to_string(&input_path)
            .expect("Unable to read file");

        serde_json::from_str::<FlxStruct>(&nfx_db).unwrap()
    };

    let topic    = "Avail2";
    let tagunits = "site";
    let tag      = "01";
    let units    = "avail";
    let measure  = 1.0;

    let now_dur = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n,
        Err(_) => panic!("Now outside LOQ."),
    };

    let _my_result = block_on(example(&nfx_db.url, &nfx_db.org, &nfx_db.token, &nfx_db.bucket, &topic, &tagunits, & tag, &units, measure, &now_dur));

    Ok(())
}
