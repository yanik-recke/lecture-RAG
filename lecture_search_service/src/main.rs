use crate::lecturestore::lecture_store_client::LectureStoreClient;
use crate::lecturestore::lecture_store_server::LectureStore;
use crate::lecturestore::{
    AddSummaryEmbeddingReq, AddSummaryEmbeddingRes, AddTranscriptEmbeddingReq,
    AddTranscriptEmbeddingRes, SimilaritySearchReq, SimilaritySearchRes,
};
use tonic::transport::Channel;
use tonic::{Request, Response, Status};

pub mod lecturestore {
    tonic::include_proto!("lecturestore");
}

fn main() {
    println!("Hello, world!");
}

struct LectureSearchService {
    client: LectureStoreClient<Channel>,
}

// #[tonic::async_trait]
