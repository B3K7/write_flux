/* write_flux/src/main.rs. */

use clap::Parser;
use clap_verbosity_flag::Verbosity;
use futures::executor::block_on;
use reqwest::Error;
use std::fs;
use serde_derive::{Deserialize, Serialize};
use chrono::DateTime;



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
   /// influx self signed CA
   #[arg(short, long)]
   ca_path: String,
   #[clap(flatten)]
   verbose: Verbosity,
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
    measure: i64,
    label : Option<String>,
    datetime : Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct MeasureStruct {
    topic: String,
    tagunits: String,
    units: String,
    _records: Vec<DataStruct>,
}

async fn wr_nflx_msg( target_path : &str, measurement_path : &str, ca_path: &str ) -> Result<(), Box<dyn std::error::Error>> {
    // send message

    use futures::prelude::*;
    use influxdb2::models::DataPoint;
    use influxdb2::models::data_point::DataPointBuilder;
    use influxdb2::models::data_point::DataPointError;
    use influxdb2::Client;
    //use time::{format_description::well_known::Rfc3339, PrimitiveDateTime, UtcOffset};


    //ingest target endpoint
    let endpoint = {
        let endpoint = fs::read_to_string(target_path)
            .expect("Unable to read file");
        serde_json::from_str::<FlxStruct>(&endpoint).unwrap()
    };

    //let client = Client::new(&endpoint.url, &endpoint.org, &endpoint.token);
    let client = Client::new_root_ca_pem(endpoint.url, endpoint.org, endpoint.token, ca_path);

    // ingest measurements
    // todo limit size main memory
    let measurement = {
        let measurement = fs::read_to_string(measurement_path)
            .expect("Unable to read file");
        serde_json::from_str::<MeasureStruct>(&measurement).unwrap()
    };

    let mut points = Vec::new();


    // marshall message
    for (i,item) in measurement._records.iter().enumerate() {

        let mut pb = DataPointBuilder::default();

        pb.measurement(&measurement.topic.clone());
        pb.tag(measurement.tagunits.clone(), item.tag.clone());
        pb.field(measurement.units.clone(), item.measure);

        if item.datetime.is_some() {
            let dt =  DateTime::parse_from_rfc3339(item.datetime.as_ref().unwrap()).unwrap();
            pb.timestamp(dt.timestamp()*1_000_000_000);

            //https://stackoverflow.com/questions/74935683/convert-utc-rfc3339-timestamp-to-local-time-with-the-time-crate
            //OffsetDateTime::parse(iter.datetime.as_ref().unwrap(),&time_rfc3389);
                // Parse the given zulu paramater.
            //let zulu : &String=  iter.datetime.as_ref().unwrap();

            // Determine Local TimeZone
            //let utc_offset = UtcOffset::current_local_offset().unwrap();
            /*{
                Ok(utc_offset) => utc_offset,
                Err(..) => return zulu.to_owned(),
            }; */

            //let zulu_parsed = PrimitiveDateTime::parse(zulu, &Rfc3339).unwrap().assume_utc();
            /*{
                Ok(zulu_parsed) => zulu_parsed.assume_utc(),
                Err(..) => return zulu.to_owned(),
            };*/

            // Convert zulu to local time offset.
            //let parsed = zulu_parsed.to_offset(utc_offset).unix_timestamp();

            //let dt =  DateTime::parse_from_rfc3339(iter.datetime.as_ref().unwrap()).unwrap();
            //pb.timestamp(parsed);
        }
        if item.label.is_some() {
            let  label=  item.label.as_ref().unwrap();
            pb.tag(  "label", label.clone());
        }

        let pr : Result<DataPoint, DataPointError> = pb.build();

        //println!("{:?}",point);
        let point: DataPoint = pr.unwrap();
        points.push(point.to_owned());

        if 99999 == i % 10000 {
            log::debug!("point vec: {:#?}", &points);
            client.write(&endpoint.bucket, stream::iter(points)).await?;
            points = Vec::new();
        }

    }

    log::debug!("point vec: {:#?}", &points);

    //send message
    Ok(client.write(&endpoint.bucket, stream::iter(points)).await?)
}

#[tokio::main]
async fn main() -> Result<(), Error>  {
    /* main routine */

    let args = Args::parse();
    let target_path = args.target_json;
    let measurement_path = args.measurement_json;
    let ca_path = args.ca_path;

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    //write message
    let my_result = block_on(wr_nflx_msg(&target_path, &measurement_path, &ca_path));

    my_result.unwrap();
    Ok(())
}

