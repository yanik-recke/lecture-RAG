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
    def imageName = "lr-${env.REGISTRY}/${env.REGISTRY_NAMESPACE}/${serviceName}"
    
    echo "Building ${imageName}:${version}"
    
    docker.build(
        "${imageName}:${version}",
        "./${serviceDir}"
    )
    
    // Tag appropriately based on build type
    if (env.IS_RELEASE == 'true') {
        sh "docker tag ${imageName}:${version} ${imageName}:latest"
        echo "✓ Tagged as 'latest' (release build)"
    } else {
        sh "docker tag ${imageName}:${version} ${imageName}:${env.BRANCH_NAME}-snapshot-latest"
        echo "✓ Tagged as '${env.BRANCH_NAME}-snapshot-latest'"
    }
}

/**
 * Push a Docker image to registry
 */
def pushService(String serviceName, String version) {
    def imageName = "${env.REGISTRY}/${env.REGISTRY_NAMESPACE}/${serviceName}"
    
    // Push version tag
    sh "docker push ${imageName}:${version}"
    echo "✓ Pushed ${imageName}:${version}"
    
    // Push additional tags
    if (env.IS_RELEASE == 'true') {
        sh "docker push ${imageName}:latest"
        echo "✓ Pushed ${imageName}:latest"
    } else {
        sh "docker push ${imageName}:${env.BRANCH_NAME}-snapshot-latest"
        echo "✓ Pushed ${imageName}:${env.BRANCH_NAME}-snapshot-latest"
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
            name: 'service-database',
            dir: 'service_database',
            envVarPrefix: 'DB'
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