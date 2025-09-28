package de.service_retrieval.service;

import de.service_retrieval.model.DocumentDTO;
import io.milvus.client.MilvusServiceClient;
import io.milvus.param.collection.HasCollectionParam;
import io.milvus.v2.service.vector.request.SearchReq;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.ai.vectorstore.SearchRequest;
import org.springframework.ai.vectorstore.VectorStore;
import org.springframework.ai.vectorstore.milvus.MilvusVectorStore;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.Optional;

@Service
public class RetrievalService {

    private static final Logger log = LoggerFactory.getLogger(RetrievalService.class);

    private final MilvusVectorStore vectorStore;


    public RetrievalService(MilvusVectorStore vectorStore) {
        this.vectorStore = vectorStore;
    }

    public List<DocumentDTO> similaritySearch(String module, String prompt) {
        Optional<MilvusServiceClient> nativeClient = vectorStore.getNativeClient();

        if (nativeClient.isPresent()) {
            MilvusServiceClient client = nativeClient.get();
            HasCollectionParam params = HasCollectionParam.newBuilder().withCollectionName(module).build();

            /* TODO
             *  create vector embeddings via embedding endpoint, then create Search Params
             *  and perform similarity search */
//            SearchReq searchReq = SearchReq.builder()
//                    .collectionName(module)
//                    .
            if (client.hasCollection(params).getData()) {

            } else {
                log.warn("No collection with name {}", module);
            }
        } else {
            log.error("Cannot get native client from vector store");
            return null;
        }

        return null;
    }
}
