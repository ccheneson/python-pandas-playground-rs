# python-pandas-playground

Ce projet consiste à créer une application qui permettra

* à un configurateur d' envoyer du code python et d' en faire une API
* à un utilisateur d' éxecuter cette API

## Stack technique

* Rust/axum utilisés pour la partie API
* Vue.js pour le développement de la partie front
* Docker pour le sandbox: L' image utilisé est [amancevice/pandas](https://hub.docker.com/r/amancevice/pandas). If you decide to take another image (i.e. make your own image), you can set it via the env variable `DOCKER_IMAGE`

Docker est utilisé comme sandbox car il
* est un choix populaire (et prouvé) dans le monde des conteneurs
* a un grand choix d' image
* permet de créer ses propres images (i.e. Dockerfile)
* possède une bonne sécurité par défaut (plus d' info [ici](https://docs.docker.com/engine/security/#:~:text=this%20blog%20post%20.-,Conclusions,or%20another%20appropriate%20hardening%20system.))
> Processes running within a container cannot see, and even less affect, processes running in another container, or in the host system.
* open-source

## Prérequis
* Docker installé
* Configurer votre envrionment pour pouvoir executer `docker` en tant qu' utilisateur (plus d 'info [ici](https://docs.docker.com/engine/install/linux-postinstall/))



## Structure

`api` : le code de l' API en Rust
`www` : le code du front-end en Vue.js

`01-build.sh`   : script pour builder l' application finale qui se trouvera dans un répertoire `app`
`02-run.sh`     : démarre l' application


## Utilisation
Dans le répertoire du projet, à partir d' un terminal:

1- Tapez  `./01-build.sh`  
2- Tapez `./02-run.sh`

Assurez vous que ces 2 scripts sont exécutables. Si ils ne le sont pas, exécutez `chmod u+x  01-build.sh 02-run.sh`