package de.bff.config;

import io.grpc.ManagedChannel;
import io.grpc.netty.NettyChannelBuilder;
import lectureservice.LectureServiceGrpc;
import metadataservice.MetadataServiceGrpc;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;


@Configuration
public class GrpcClientConfig {

    @Value("${grpc.lecture-service.host:localhost}")
    private String lectureServiceHost;

    @Value("${grpc.lecture-service.port:50051}")
    private int lectureServicePort;

    @Value("${METADATA_SERVICE_HOST:localhost}")
    private String metadataServiceHost;

    @Value("${METADATA_SERVICE_PORT:40042")
    private int metadataServicePort;

    public static int GRPC_MESSAGE_SIZE = 100_000_000;

    @Bean
    public ManagedChannel metadataServiceChannel() {
        return NettyChannelBuilder
                .forAddress(this.metadataServiceHost, this.metadataServicePort)
                .usePlaintext()
                .maxInboundMessageSize(GRPC_MESSAGE_SIZE)
                .maxInboundMetadataSize(GRPC_MESSAGE_SIZE)
                .build();
    }

    @Bean
    public MetadataServiceGrpc.MetadataServiceBlockingStub metadataServiceBlockingStub(ManagedChannel metadataServiceChannel) {
        return MetadataServiceGrpc.newBlockingStub(metadataServiceChannel)
                .withMaxInboundMessageSize(GRPC_MESSAGE_SIZE)
                .withMaxOutboundMessageSize(GRPC_MESSAGE_SIZE);
    }

    @Bean
    public ManagedChannel lectureServiceChannel() {

        return NettyChannelBuilder
                .forAddress(lectureServiceHost, lectureServicePort)
                .usePlaintext()
                .maxInboundMessageSize(GRPC_MESSAGE_SIZE)
                .maxInboundMetadataSize(GRPC_MESSAGE_SIZE)
                .build();
    }

    @Bean
    public LectureServiceGrpc.LectureServiceBlockingStub lectureServiceStub(ManagedChannel lectureServiceChannel) {
        return LectureServiceGrpc.newBlockingStub(lectureServiceChannel)
                .withMaxInboundMessageSize(GRPC_MESSAGE_SIZE)
                .withMaxOutboundMessageSize(GRPC_MESSAGE_SIZE);
    }
}