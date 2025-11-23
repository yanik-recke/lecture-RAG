/**
 * Determine version for a service based on whether it's a release or snapshot build
 */
def determineVersion(String serviceDir) {
    if (env.IS_RELEASE == 'true') {
        return "${params.MANUAL_VERSION}-RELEASE"
    } else {
        def version = readFile("${serviceDir}/VERSION").trim()
        return "${version}-SNAPSHOT"
    }
}

/**
 * Build a Docker image for a service
 */
def buildService(String serviceName, String serviceDir, String version) {
    echo 'Building and pushing Docker image using buildctl...'
    
    container('jenkins-agent-buildkit') {
        sh """
            buildctl build \
                --frontend=dockerfile.v0 \
                --local context=. \
                --local dockerfile= ./${serviceDir}/Dockerfile \
                --platform linux/amd64 \
                --output type=image,name=${REPO}/${serviceName}:SNAPSHOT-${version},push=true
        """
    }
}

/**
 * Detect changes for a list of services
 */
def detectChanges(List<String> services, String changesOutput) {
    def changedServices = [:]
    
    services.each { service ->
        def serviceChanged = (env.IS_MANUAL_TRIGGER == 'true') || changesOutput.contains("${service}/")
        changedServices[service] = serviceChanged
        
        // Set environment variable for each service
        def envVarName = "${service.toUpperCase().replace('/', '_')}_CHANGED"
        env."${envVarName}" = serviceChanged ? 'true' : 'false'
        
        echo "${service}: ${serviceChanged ? 'CHANGED' : 'no changes'}"
    }
    
    return changedServices
}

def getServiceConfig() {
    return [
        [
            name: 'lecture_search_service',
            dir: 'lecture_search_service',
            envVarPrefix: 'FACILITATOR'
        ]
        // [
        //     name: 'service-embedding',
        //     dir: 'service_embedding',
        //     envVarPrefix: 'EMBEDDING'
        // ],
        // [
        //     name: 'service-frontend',
        //     dir: 'service_frontend',
        //     envVarPrefix: 'FRONTEND'
        // ],
        // [
        //     name: 'service-retrieval',
        //     dir: 'service_retrieval',
        //     envVarPrefix: 'RETRIEVAL'
        // ],
        // [
        //     name: 'service-speechtotext',
        //     dir: 'service_speechtotext',
        //     envVarPrefix: 'SPEECHTOTEXT'
        // ],
        // [
        //     name: 'vectorstore-milvus',
        //     dir: 'vectorstore_milvus',
        //     envVarPrefix: 'MILVUS'
        // ]
    ]
}

return this