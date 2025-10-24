pipeline {
    agent any
    
    parameters {
        string(
            name: 'MANUAL_VERSION',
            defaultValue: '',
        )
        booleanParam(
            name: 'FORCE_BUILD_ALL',
            defaultValue: false,
        )
    }
    
    environment {
        REGISTRY = 'ghcr.io'
        REGISTRY_NAMESPACE = 'yanik-recke'
        REGISTRY_CREDENTIAL = 'docker-registry-credentials'
    }
    
    stages {
        stage('Setup') {
            steps {
                script {
                    buildFunctions = load 'jenkins/BuildFunctions.groovy'
                    
                    def isManualTrigger = currentBuild.getBuildCauses('hudson.model.Cause$UserIdCause').size() > 0
                    env.IS_MANUAL_TRIGGER = isManualTrigger ? 'true' : 'false'
                    
                    env.IS_RELEASE = (isManualTrigger && params.MANUAL_VERSION?.trim()) ? 'true' : 'false'
                    
                    echo "═══════════════════════════════════════"
                    echo "Manual trigger: ${env.IS_MANUAL_TRIGGER}"
                    echo "Release build: ${env.IS_RELEASE}"
                    if (env.IS_RELEASE == 'true') {
                        echo "Release version: ${params.MANUAL_VERSION}"
                    }
                    echo "═══════════════════════════════════════"
                }
            }
        }
        
        stage('Detect Changes') {
            steps {
                script {
                    def changes = sh(
                        script: "git diff --name-only HEAD~1 HEAD || git diff --name-only HEAD",
                        returnStdout: true
                    ).trim()
                    
                    def services = buildFunctions.getServiceConfig()
                    def serviceDirs = services.collect { it.dir }
                    
                    echo "Detecting changes..."
                    buildFunctions.detectChanges(serviceDirs, changes)
                }
            }
        }
        
        stage('Determine Versions') {
            steps {
                script {
                    def services = buildFunctions.getServiceConfig()
                    
                    echo "Determining versions for changed services..."
                    services.each { service ->
                        def changedEnvVar = "${service.dir.toUpperCase().replace('/', '_')}_CHANGED"
                        
                        if (env."${changedEnvVar}" == 'true') {
                            def version = buildFunctions.determineVersion(service.dir)
                            def versionEnvVar = "${service.envVarPrefix}_VERSION"
                            env."${versionEnvVar}" = version
                            
                            echo "${service.name}: ${version}"
                        }
                    }
                }
            }
        }
        
        stage('Build Services') {
            steps {
                script {
                    def services = buildFunctions.getServiceConfig()
                    def parallelBuilds = [:]
                    
                    services.each { service ->
                        def changedEnvVar = "${service.dir.toUpperCase().replace('/', '_')}_CHANGED"
                        
                        if (env."${changedEnvVar}" == 'true') {
                            def serviceName = service.name
                            def serviceDir = service.dir
                            def versionEnvVar = "${service.envVarPrefix}_VERSION"
                            def version = env."${versionEnvVar}"
                            
                            parallelBuilds["Build ${serviceName}"] = {
                                buildFunctions.buildService(serviceName, serviceDir, version)
                            }
                        }
                    }
                    
                    if (parallelBuilds.size() > 0) {
                        echo "Building ${parallelBuilds.size()} service(s) in parallel..."
                        parallel parallelBuilds
                    } else {
                        echo "No services to build"
                    }
                }
            }
        }
        
        stage('Push to Registry') {
            steps {
                script {
                    def services = buildFunctions.getServiceConfig()
                    
                    docker.withRegistry("https://${REGISTRY}", REGISTRY_CREDENTIAL) {
                        services.each { service ->
                            def changedEnvVar = "${service.dir.toUpperCase().replace('/', '_')}_CHANGED"
                            
                            if (env."${changedEnvVar}" == 'true') {
                                def version = env."${service.envVarPrefix}_VERSION"
                                buildFunctions.pushService(service.name, version)
                            }
                        }
                    }
                }
            }
        }
    }
    
    post {
        always {
            sh 'docker system prune -f'
        }
        success {
            script {
                def services = buildFunctions.getServiceConfig()
                def builtServices = []
                
                services.each { service ->
                    def changedEnvVar = "${service.dir.toUpperCase().replace('/', '_')}_CHANGED"
                    if (env."${changedEnvVar}" == 'true') {
                        def version = env."${service.envVarPrefix}_VERSION"
                        builtServices.add("${service.name}:${version}")
                    }
                }
                
                echo "════════════════════════════════════════"
                echo "✓ Build completed successfully!"
                if (builtServices.size() > 0) {
                    echo "Built services:"
                    builtServices.each { echo "  - ${it}" }
                    
                    if (env.IS_RELEASE == 'true') {
                        echo ""
                        echo "⚠️  IMPORTANT: Update VERSION files for released services"
                    }
                } else {
                    echo "No services were built (no changes detected)"
                }
                echo "════════════════════════════════════════"
            }
        }
        failure {
            echo "✗ Build failed!"
        }
    }
}