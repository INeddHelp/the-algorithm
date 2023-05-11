use std::collections::BTreeMap;
use std::collections::BTreeSet;

use bpr_thrift::data::DataRecord;
use bpr_thrift::prediction_service::BatchPredictionRequest;
use thrift::OrderedFloat;

use thrift::protocol::TBinaryInputProtocol;
use thrift::protocol::TSerializable;
use thrift::transport::TBufferChannel;
use thrift::Result;

fn main() {
    let data_path = "/tmp/current/timelines/output-1";
    let bin_data: Vec<u8> = std::fs::read(data_path).expect("Could not read file!");

    println!("Length : {}", bin_data.len());

    let mut bc = TBufferChannel::with_capacity(bin_data.len(), 0);

    bc.set_readable_bytes(&bin_data);

    let mut protocol = TBinaryInputProtocol::new(bc, true);

    let result: Result<BatchPredictionRequest> =
        BatchPredictionRequest::read_from_in_protocol(&mut protocol);

    match result {
        Ok(bpr) => logBP(bpr),
        Err(err) => println!("Error {}", err),
    }
}

fn logBP(bpr: BatchPredictionRequest) {
    println!("-------[OUTPUT]---------------");
    println!("data {:?}", bpr);
    println!("------------------------------");

    /*
    let common = bpr.common_features;
    let recs = bpr.individual_features_list;

    println!("--------[Len : {}]------------------", recs.len());

    println!("-------[COMMON]---------------");
    match common {
      Some(dr) => logDR(dr),
      None => println!("None"),
    }
    println!("------------------------------");
    for rec in recs {
      logDR(rec);
    }
    println!("------------------------------");
    */
}

fn logDR(dr: DataRecord) {
    println!("--------[DR]------------------");

    if let Some(bf) = dr.binary_features {
        logBin(bf)
    }

    if let Some(cf) = dr.continuous_features {
        logCF(cf)
    }
    println!("------------------------------");
}

fn logBin(bin: BTreeSet<i64>) {
    println!("B: {:?}", bin)
}

fn logCF(cf: BTreeMap<i64, OrderedFloat<f64>>) {
    for (id, fs) in cf {
        println!("C: {} -> [{}]", id, fs);
    }
}
