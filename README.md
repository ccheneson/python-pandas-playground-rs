# python-pandas-playground

Ce projet consiste à créer une application qui permettra

* à un configurateur d' envoyer du code python et d' en faire une API
* à un utilisateur d' éxecuter cette API

## Exigences technique:
• Utilisation de Axum pour le framework web.
• Le code Python doit pouvoir utiliser la bibliothèque pandas.
• Suivre les principes du clean code / YAGNI : implémenter uniquement ce qui est nécessaire
pour que cela fonctionne.

## Stack technique

* Rust/axum utilisés pour la partie API
* Vue.js pour le développement de la partie front
* Docker pour le sandbox: L' image utilisée est [amancevice/pandas](https://hub.docker.com/r/amancevice/pandas). Si vous préferez utiliser une autre image (i.e. votre propre image créée à partir d' un Dockerfile), vous pouvez utiliser la variable d' environment `DOCKER_IMAGE`

Docker est utilisé comme sandbox car il
* est un choix populaire (et prouvé) dans le monde des conteneurs
* a un grand choix d' image
* permet de créer ses propres images (i.e. Dockerfile)
* possède une bonne sécurité par défaut (plus d' info [ici](https://docs.docker.com/engine/security/#:~:text=this%20blog%20post%20.-,Conclusions,or%20another%20appropriate%20hardening%20system.))
> Processes running within a container cannot see, and even less affect, processes running in another container, or in the host system.
* open-source

## Prérequis
* Docker installé
* Configurer votre environment pour pouvoir executer `docker` en tant qu' utilisateur (plus d 'info [ici](https://docs.docker.com/engine/install/linux-postinstall/)) et n' aura donc pas besoin de `sudo`.



## Structure

`api` : le code de l' API en Rust

`www` : le code du front-end en Vue.js

`01-build.sh`   : script pour builder l' application finale qui se trouvera dans un répertoire `app`

`02-run.sh`     : démarre l' application


## Utilisation
Dans le répertoire du projet, à partir d' un terminal:

1- Tapez  `./01-build.sh`  
2- Tapez `./02-run.sh`

Assurez vous que ces 2 scripts soient exécutables. 

Si ils ne le sont pas, exécutez `chmod u+x  01-build.sh 02-run.sh`.


## Choix technique
Comme mentionné dans la section **Stack technique**, Docker est un choix populaire dans le monde des conteneurs.

1 inconvénient pour notre application: Si l' image n' est pas présente dans notre répository Docker local, la toute 1ère requête prendra le temps de téléchargement de l' image + temps d' éxecution.

Pour les requêtes suivantes, la requête ne prendrait que le temps d' éxecution.


#### Backend
Le service se compose de 3 parties:
- `http/handlers` où se trouve la logique des endpoints
- `repositories` qui servira de persistence. Pour cette application, nous utiliserons un simple HashMap. Pour notre cas, nous pouvons utiliser les types directement sans utiliser de `Result` pour la gestion d' erreur. 
- `sandbox` qui se chargera de l' execution dans le docker. Le sandbox éxecute une commande qui peut échouer tant que la commande `docker` que le code python. C' est pour cela que l' on utilisera un type `Result` pour la gestion d' erreur


#### Frontend
La partie interface est développée en Vue.js et sera servie par Rust/axum au lieu de node
