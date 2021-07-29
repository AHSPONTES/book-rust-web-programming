#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

scp -i "./rust_app.pem" ./docker-compose.yml ec2-user@18.118.20.124:/home/ec2-user/docker-compose.yml

scp -i "./rust_app.pem" -r ./nginx ec2-user@18.118.20.124:/home/ec2-user/nginx

ssh -i "./rust_app.pem" -t ec2-user@18.118.20.124 << EOF
    docker-compose stop
    docker container rm rust_app
    docker image rm ahsp92/actix_web_application
    docker-compose up -d

    docker exec "rust_app" bash -c "echo 'DATABASE_URL=postgres://username:password@todo.XXXXXXXXX.us-east-2.rds.amazonaws.com:5432/todo' > .env"

    docker container exec -t rust_app diesel setup
    docker container exec -t rust_app diesel migration run
        rm -r nginx/
        rm docker-compose.yml
EOF

curl --header "Content-Type: application/json" --request POST --data '{"name":"test", "email":"test", "password":"test"}' 18.118.20.124/api/v1/user/create