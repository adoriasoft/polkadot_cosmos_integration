pipeline {
  agent any
  environment {
    BRANCH = "add_ci_jenkins"
    GIT_CREDENTIALS = "adoriasoft-github"
    GIT_REPO = "git@github.com:adoriasoft/polkadot_cosmos_integration.git"
  }
  stages {
    stage("GitHub checkout") {
      steps {
        git branch: env.BRANCH, credentialsId: env.GIT_CREDENTIALS, url: env.GIT_REPO, poll: false
  }
}
    stage('Setup environment') {
      steps {
        sh "curl https://getsubstrate.io -sSf | bash -s -- --fast"
        sh "source ~/.cargo/env"
        sh "rustup default stable"
        sh "rustup update nightly"
        sh "rustup update stable"
        sh "rustup target add wasm32-unknown-unknown --toolchain nightly"
        sh "rustup update"
    }
  }
  stage('Build project') {
    steps { dir('substrate') {
        sh "cargo clean"
        sh "cargo build --release"
    }
  }
}
    stage('Run tests') {
      steps {
        dir('substrate') { sh "cargo test --all" }
    }
  }
}
}
