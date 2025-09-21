# Thiery-LB - Cahier des Charges
Thiery-LB est un projet de load balancer écrit en Rust, conçu pour repartir de manière optimale la charge réseau entre plusieurs serveurs. Ce document détaille les objectifs, les fonctionnalités attendues et les contraintes techniques du projet.

## Utilisation
L'objectif pour l'utilisateur est de pouvoir très simplement configurer et déployer son load balancer en CLI.

## Scénario d'Utilisation
1. L'utilisateur télécharge la dernière version de l'exécutable depuis le repo GitHub.
2. Il lance l'exécutable et se retrouve avec une interface en ligne de commande.
3. Il configure ses serveurs backend : Il peut ajouter, modifier ou supprimer des serveurs.
4. Il choisit le port d'écoute du load balancer.
5. Il sélectionne l'algorithme de répartition de charge (TBD).
6. Il démarre le load balancer.

## Futures Améliorations
- Dockerisation pour un déploiement simplifié.
- Interface Web pour la configuration et le monitoring.
- Support de protocoles supplémentaires (HTTPS, WebSocket).
- Implémentation de la stratégie de Sticky Sessions : un client est toujours redirigé vers le même serveur.

## Fonctionnalités Principales

### Interface (dans l'ordre d'évolution)
1. Interface en ligne de commande (CLI).
2. Menu interactif / Monitoring avec tui-rs ou Ratatui.
3. Monitoring web dockerisé.

### Fichier de ConfigurationTODOs
- Format custom de la config.
- Exemple de fichier de config:
```
port=8080
algorithm=round-robin
server1=127.0.0.1:3000
server2=127.0.0.1:3001
```
- Parsing du fichier de config.

### Algorithmes de Répartition de Charge
- On fait un mixups d'enfer mon gars.

## A faire
- [ ] Parsing des arguments CLI.
- [ ] Menu interactif en CLI. (tui-rs?, Ratatui?)
- [ ] Implémentation du serveur TCP.
- [ ] Initialisation de la configuration.
- [ ] Load/Parse/Save fichier de configuration.
- [ ] Interfacage des algorithmes de répartition.
- [ ] Implémentation des algorithmes de répartition.
- [ ] Gestion des erreurs et logs.
- [ ] Tests unitaires et d'intégration avec un docker compose.
