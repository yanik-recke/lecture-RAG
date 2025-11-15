package de.bff.config;

import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;
import io.grpc.netty.NettyChannelBuilder;
import lectureservice.LectureServiceGrpc;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;


@Configuration
public class GrpcClientConfig {

    @Value("${grpc.lecture-service.host:localhost}")
    private String lectureServiceHost;

    @Value("${grpc.lecture-service.port:50051}")
    private int lectureServicePort;

    public static int GRPC_MESSAGE_SIZE = 100_000_000;

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