package de.bff.service;

import metadataservice.MetadataServiceGrpc;
import metadataservice.MetadataServiceOuterClass;
import org.springframework.stereotype.Service;

@Service
public class ModuleService {

    private final MetadataServiceGrpc.MetadataServiceBlockingStub metadataServiceBlockingStub;

    public ModuleService(MetadataServiceGrpc.MetadataServiceBlockingStub metadataServiceBlockingStub) {
        this.metadataServiceBlockingStub = metadataServiceBlockingStub;
    }

    public MetadataServiceOuterClass.GetModulesNamesRes getModulesNames() {
        return this.metadataServiceBlockingStub.getModulesNames(MetadataServiceOuterClass.GetModulesNamesReq.newBuilder().build());
    }



}
