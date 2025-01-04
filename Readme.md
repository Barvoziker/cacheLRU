# Bienvenu sur mon projet de cache LRU en RUST

## Introduction
Ce projet est une implémentation de cache LRU en RUST. Le cache LRU est un cache qui garde les éléments les plus récemment utilisés. Lorsqu'un élément est ajouté au cache, il est ajouté en tête de liste. Lorsqu'un élément est accédé, il est déplacé en tête de liste. Lorsque le cache est plein, l'élément le moins récemment utilisé est supprimé.

## Génération de la documentation
Pour générer la documentation, il suffit de lancer la commande suivante:
```bash
cargo doc --open
```
Grâce à cette commande, la documentation sera générée et ouverte dans le navigateur par défaut.

## Auteurs
Ce projet a été réalisé par **Mathis BUCHET**

## Sources
- [Documentation officielle de RUST](https://doc.rust-lang.org/book/)
- [Documentation de la librairie standard de RUST](https://doc.rust-lang.org/std/)