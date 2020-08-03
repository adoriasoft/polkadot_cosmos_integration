pipeline {
  agent any
  environment {
    BRANCH = "master"
    GIT_CREDENTIALS = "adoriasoft-github"
    GIT_REPO = "git@github.com:adoriasoft/polkadot_cosmos_integration.git"
  }
  stages {
    stage("GitHub checkout") {
      steps {
        git branch: env.BRANCH, credentialsId: env.GIT_CREDENTIALS, url: env.GIT_REPO, poll: false
      }
    }
    stage('Build docker image') {
      steps {
        sh "docker build --no-cache -t ${env.JOB_NAME.toLowerCase()}-${env.BUILD_NUMBER} ."
      }
    }
    stage("Run tests") {
      steps {
        sh "docker run -i \
        --rm ${env.JOB_NAME.toLowerCase()}-${env.BUILD_NUMBER} \
        cargo test --all"
      }
    }
    stage("Clean docker artifacts") {
      steps {
        sh "docker rmi ${env.JOB_NAME.toLowerCase()}-${env.BUILD_NUMBER}"
      }
    }
  }
    post {
      always {
        deleteDir()
      }
    }
}
