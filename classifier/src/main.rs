use crate::repo::mongodb::MongoRepo;
use crate::types::classification::Classification::UNCLASSIFIED;
use crate::types::classifier::Classifier;

//use kafka::producer::{Producer, Record, RequiredAcks};
pub mod types;
pub mod repo;
pub mod conditions;
#[tokio::main]
async fn main() {
     const DATABASE:&str = "Classifier";
     const COLLECTION:&str ="Rules";

    
  

    // let classifier = Classifier{
    //     condition:conditions::condition1.clone(),
    //     classification:UNCLASSIFIED,
    //     record_type:"cdr".to_string(),
    //     order:1,
    // };

    // let repo = MongoRepo::new("mongodb://amd:ak@localhost:27017".to_string()).await.unwrap();
    // let _insert = repo.insert_record(DATABASE,COLLECTION,classifier).await.unwrap();

    // let gettt = repo.get_by_record_type(DATABASE,DATABASE,"cdr").await.unwrap();
    // for i in gettt {
    //     println!("{:?}",i);
    // }


//     let classifier_json = serde_json::to_string_pretty(&classifier).unwrap();
//    println!("Serialized classifier:\n: {}", classifier_json);

    // let  buf = serde_json::to_vec(&record).unwrap();
    // let  producer =
    //     Producer::from_hosts(vec!("localhost:9092".to_owned()))
    //         .with_ack_timeout(Duration::from_secs(1))
    //         .with_required_acks(RequiredAcks::One)
    //         .create();

    // match producer{
    //     Ok(mut producer) =>{
    //         let result = &producer.send(&Record::from_value("my-topic", buf));
    //         match result {
    //             Ok(_) => {
    //                 println!("OK");
    //             }
    //             Err(_) => {
    //                 println!("Error: ");
    //             }
    //         }
    //     }
    //     _ => println!("error")
    // }




}
