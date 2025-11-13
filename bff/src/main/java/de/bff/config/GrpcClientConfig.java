package de.bff.config;

import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;
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

    @Bean
    public ManagedChannel lectureServiceChannel() {

        return ManagedChannelBuilder
                .forAddress(lectureServiceHost, lectureServicePort)
                .usePlaintext()
                .build();
    }

    @Bean
    public LectureServiceGrpc.LectureServiceBlockingStub lectureServiceStub(ManagedChannel lectureServiceChannel) {
        return LectureServiceGrpc.newBlockingStub(lectureServiceChannel);
    }
}