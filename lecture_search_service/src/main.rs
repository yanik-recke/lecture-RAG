mod whisperservice;

use crate::lecturestore_mod::lecture_store_client::LectureStoreClient;
use crate::lecturestore_mod::lecture_store_server::LectureStore;
use tonic::transport::Channel;

pub mod lectureservice {
    tonic::include_proto!("lectureservice");
}

pub mod lecturestore_mod {
    tonic::include_proto!("lecturestore");
}

pub mod embeddingservice_mod {
    tonic::include_proto!("embeddingservice");
}

pub mod completionservice_mod {
    tonic::include_proto!("completionservice");
}

pub mod summaryservice_mod {
    tonic::include_proto!("summaryservice");
}

pub mod whisperservice_mod {
    tonic::include_proto!("whisperservice");
}

// #[tonic::async_trait]
fn main() {
    println!("Hello, world!");
}

struct LectureSearchService {
    client: LectureStoreClient<Channel>,
}
