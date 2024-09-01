# python-pandas-playground

Ce projet consiste à créer une application qui permettra

* à un configurateur d' envoyer du code python et d' en faire une API
* à un utilisateur d' éxecuter cette API

## Stack technique

* Rust/axum utilisés pour la partie API
* Vue.js pour le développement de la partie front
* Docker pour le sandbox

Docker est utilisé comme sandbox car il
* est un choix populaire (et prouvé) dans le monde des conteneurs
* a un grand choix d' image
* permet de créer ses propres images (i.e. Dockerfile)
* possède une bonne sécurité par défaut (plus d' info [ici](https://docs.docker.com/engine/security/#:~:text=this%20blog%20post%20.-,Conclusions,or%20another%20appropriate%20hardening%20system.))
> Processes running within a container cannot see, and even less affect, processes running in another container, or in the host system.
* open-source