node {
    def commit_id;
    def RUST = '$HOME/.cargo/bin/';
    def RUST_TESTING_FLAGS ='RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads" CARGO_INCREMENTAL=0';
    stage('Preparation') {
        checkout scm
        sh "git rev-parse --short HEAD > .git/commit-id"
        commit_id = readFile('.git/commit-id').trim()
        sh "${RUST}cargo --version"
    }
    stage('Build') {
        sh "${RUST}cargo build"
    }
    stage('Unit tests') {
        sh "${RUST_TESTING_FLAGS} ${RUST}cargo test --lib"
    }
    stage('Integration tests') {
        sh "${RUST_TESTING_FLAGS} ${RUST}cargo test --test integration_tests"
    }
    if(env.BRANCH_NAME == 'master') {
        stage('Publish crate') {
            sh "echo publish"
            // sh "cargo publish"
        }
    }
    stage('Clean up') {
        sh "rm -r ./target"
    }
}