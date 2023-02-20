/* infx_w/src/main/rs. */

use clap::Parser;
use futures::executor::block_on;
use reqwest::Error;
use std::fs;
use serde_derive::{Deserialize, Serialize};

/// Send point measurement(s) to influxdb2 target
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// endpoint target
   #[arg(short, long)]
   target_json: String,
   /// influx measurements
   #[arg(short, long)]
   measurement_json: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct FlxStruct {
    url    : String,
    org    : String,
    token  : String,
    bucket : String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataStruct {
    tag    : String,
    measure: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct MeasureStruct {
    topic: String,
    tagunits: String,
    units: String,
    _records: Vec<DataStruct>,
}

async fn wr_nflx_msg( target_path : &str, measurement_path : &str ) -> Result<(), Box<dyn std::error::Error>> {

    use futures::prelude::*;
    use influxdb2::models::DataPoint;
    use influxdb2::Client;

    //target endpoint
    let endpoint = {
        let endpoint = fs::read_to_string(target_path)
            .expect("Unable to read file");
        serde_json::from_str::<FlxStruct>(&endpoint).unwrap()
    };

    let client = Client::new(&endpoint.url, &endpoint.org, &endpoint.token);

    let measurement = {
        let measurement = fs::read_to_string(measurement_path)
            .expect("Unable to read file");
        serde_json::from_str::<MeasureStruct>(&measurement).unwrap()
    };

    let mut points = Vec::new();

    for iter in &measurement._records {
        let point =
            DataPoint::builder(&measurement.topic)
                .tag(  &measurement.tagunits, &iter.tag)
                .field(&measurement.units,    iter.measure)
                .build()?;
        points.push(point);
    }

    client.write(&endpoint.bucket, stream::iter(points)).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error>  {
    /* main routine */

    let args = Args::parse();
    let target_path = args.target_json;
    let measurement_path = args.measurement_json;

    let _my_result = block_on(wr_nflx_msg(&target_path, &measurement_path));

    Ok(())
}
