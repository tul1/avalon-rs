pipeline {
    agent { 
         docker { image 'rust:stretch' }
    }
    stages {
        stage('Rustfmt') {
            steps {
                // This step isn't currently available because
                // cargo fmt is not passing due to some technical
                // dept we've gathered.
                // TODO uncomment once cargo fmt passes. 
                // sh "cargo fmt -- --check"
                sh "echo coming soon"
            }
        }
        stage('Clippy') {
            steps {
                // TODO uncomment once cargo clippy passes. 
                // sh "cargo clippy"
                sh "echo coming soon"
            }
        }
        stage('Build') {
            steps {
                sh "cargo build"
            }
        }
        stage('Test') {
            steps {
                sh "cargo test"
            }
        }
        stage('Documentation') {
            steps {
                sh "cargo doc"
            }
        }
        stage('Publish new version') {
            when {
                expression { scm.branches[0].name == 'stable' }
            }
            steps {
                withCredentials([string(credentialsId: 'cargo_io_credential', variable: 'CARGO_IO_CREDENTIAL')]) {
                    // TODO uncomment cargo publish once we feel ready to officially deploy
                    // to crate.io by the first time. (Cargo credential is already set up).
                    sh '''
                        cargo login $CARGO_IO_CREDENTIAL
                        #cargo publish
                    '''
                }
            }
        }
        stage('Clean up') {
            steps {
               sh "cargo clean"
            }
        }
    }
}
