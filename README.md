# LoutreBase

Petit hub de liens réalisé avec Rust et Leptos, compilé en WebAssembly avec
Trunk.

## Développement avec le Dev Container

Le projet fournit une configuration de Dev Container dans `.devcontainer/`.
Utilisez Podman comme moteur de conteneurs et ouvrez le dossier du projet dans
le Dev Container depuis VS Code. L’environnement installe la cible
`wasm32-unknown-unknown` et Trunk.

Lancez le serveur de développement dans le terminal intégré de VS Code :

```sh
trunk serve --address 0.0.0.0
```

Ouvrez ensuite <http://localhost:8080>. Le port est transféré par VS Code.
Le serveur Trunk tourne dans le Dev Container : il n’est pas nécessaire de
lancer un deuxième conteneur pour le site. Après un rechargement de la fenêtre,
reconnectez-vous au Dev Container existant plutôt que d’en créer un autre.

Le Dev Container sert d’environnement de développement ; les fichiers du projet
restent dans le dossier du workspace ouvert par VS Code.

## Développement sans conteneur

Installez Rust, puis la cible WebAssembly et Trunk :

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
```

Lancez ensuite le serveur avec la même commande `trunk serve` ci-dessus.

## Compilation

```sh
trunk build --release
```

Les destinations d’exemple sont définies dans `src/main.rs`.
