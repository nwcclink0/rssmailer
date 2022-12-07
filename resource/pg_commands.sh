#!/bin/bash
display_usage() {
 echo "-s start postgrsql database"
 echo "-k stop postgrsql database"
 echo "-l login postgrsql database"
 echo "-t create table"
 echo "-r sqlx migrate run"
 echo "-d reset database"
 echo "-h help"
}

if [ $# -le 0 ]
then
  display_usage
  exit 1
fi 

if [[ ($# == "--help") || ($# == "-h") ]]
then
  display_usage
  exit 0
fi


DATABASE_URL="postgres://$POSTGRES_HOSTNAME?dbname=rssmailer&user=$POSTGRES_USER&password=$POSTGRES_PASSWORD"


start_db() {
  pg_ctl -D /home/linuxbrew/.linuxbrew/var/postgres -l /home/linuxbrew/.linuxbrew/var/postgres/server.log start
}

stop_db() {
  pg_ctl -D /home/linuxbrew/.linuxbrew/var/postgres -l /home/linuxbrew/.linuxbrew/var/postgres/server.log stop
}

login_db() {
  psql -U $POSTGRES_USER rssmailer
}

create_table() {
  sqlx migrate add account
  sqlx migrate add rssfeed 
}

run_migrate() {
  sqlx migrate run
}

create_db() {
  sqlx database create
}

reset_database() {
  echo "database url:" $DATABASE_URL
  sqlx database drop
  sqlx database create
  sqlx migrate run
}

while [ -n "$1" ]; do 
    case "$1" in
    -s)
        start_db
        shift
        ;;
    -l)
        login_db
        shift
        ;;
    -k)
        stop_db
        shift
        ;;
    -t)
        create_table
        shift
        ;;
    -r)
        run_migrate
        shift
        ;;
    -d)
        reset_database
        shift
        ;;
    *)
        echo " Option $1 not recognized";;
    esac
    shift
done
