#!/bin/bash
display_usage() {
 echo "-s start postgrsql database"
 echo "-k stop postgrsql database"
 echo "-l login postgrsql database"
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


start_db() {
  pg_ctl -D /home/linuxbrew/.linuxbrew/var/postgres -l /home/linuxbrew/.linuxbrew/var/postgres/server.log start
}

stop_db() {
  pg_ctl -D /home/linuxbrew/.linuxbrew/var/postgres -l /home/linuxbrew/.linuxbrew/var/postgres/server.log stop
}

login_db() {
  psql -U yt rssmailer
}

create_table() {
  sqlx migrate add account
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
    *)
        echo " Option $1 not recognized";;
    esac
    shift
done
