use chrono::DateTime;
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use reqwest::{Client, Certificate};
use serde::Deserialize;
use std::fs;

/// Send point measurement(s) to influxdb2 target via HTTP Line Protocol
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Endpoint target JSON
    #[arg(short, long)]
    target_json: String,
    /// Influx measurements JSON
    #[arg(short, long)]
    measurement_json: String,
    /// Influx self-signed CA path (PEM format)
    #[arg(short, long)]
    ca_path: String,
    #[clap(flatten)]
    verbose: Verbosity,
}

#[derive(Deserialize, Debug)]
struct FlxStruct {
    url: String,
    org: String,
    token: String,
    bucket: String,
}

#[derive(Deserialize, Debug)]
struct DataStruct {
    tag: String,
    measure: i64,
    label: Option<String>,
    datetime: Option<String>,
}

#[derive(Deserialize, Debug)]
struct MeasureStruct {
    topic: String,
    tagunits: String,
    units: String,
    _records: Vec<DataStruct>,
}

async fn send_batch(client: &Client, endpoint: &FlxStruct, lines: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/api/v2/write", endpoint.url.trim_end_matches('/'));
    let body = lines.join("\n");

    let res = client.post(&url)
        .query(&[
            ("org", &endpoint.org),
            ("bucket", &endpoint.bucket),
            ("precision", &"ns".to_string())
        ])
        .header("Authorization", format!("Token {}", endpoint.token))
        .body(body)
        .send()
        .await?;

    // Raise an error if the request failed (e.g. 401 Unauthorized, 400 Bad Request)
    res.error_for_status()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    // Ingest target endpoint and measurements
    let endpoint: FlxStruct = serde_json::from_str(&fs::read_to_string(&args.target_json)?)?;
    let measurement: MeasureStruct = serde_json::from_str(&fs::read_to_string(&args.measurement_json)?)?;

    // Read the custom CA certificate
    let cert_bytes = fs::read(&args.ca_path)?;
    
    // Parses the PEM file for the Rustls + aws-lc-rs backend
    let cert = Certificate::from_pem(&cert_bytes)?;

    // Build the reqwest client using the custom Root CA
    let client = Client::builder()
        .add_root_certificate(cert)
        .build()?;

    let mut lines_batch = Vec::with_capacity(10000);

    // Marshall messages to InfluxDB Line Protocol
    for (i, item) in measurement._records.into_iter().enumerate() {
        let mut line = format!("{},{}={}", measurement.topic, measurement.tagunits, item.tag);

        if let Some(label) = item.label {
            line.push_str(&format!(",label={}", label));
        }

        line.push_str(&format!(" {}={}i", measurement.units, item.measure));

        if let Some(dt_str) = item.datetime {
            let dt = DateTime::parse_from_rfc3339(&dt_str)?;
            let ns = dt.timestamp() * 1_000_000_000;
            line.push_str(&format!(" {}", ns));
        }

        lines_batch.push(line);

        // Batch write every 10,000 points
        if (i + 1) % 10000 == 0 {
            log::debug!("Writing batch of {} points", lines_batch.len());
            send_batch(&client, &endpoint, &lines_batch).await?;
            lines_batch.clear();
        }
    }

    // Write any remaining messages
    if !lines_batch.is_empty() {
        log::debug!("Writing final batch of {} points", lines_batch.len());
        send_batch(&client, &endpoint, &lines_batch).await?;
    }

    Ok(())
}
