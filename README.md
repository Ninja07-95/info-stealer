# Info Stealer 🖥️

Un outil en **Rust** pour récupérer et afficher les informations système de votre machine.

## Fonctionnalités

- 👤 Informations utilisateur (nom, username, email, langue préférée)
- 💻 Détails système (OS, version kernel, hostname)
- ⏱️ Uptime et temps de démarrage
- 🧠 Utilisation CPU, RAM et swap
- 💾 Espace disque disponible
- 🌐 Interfaces réseau et adresses IP
- 📊 Nombre de processus actifs

## Installation

```bash
git clone https://github.com/Ninja07-95/info-stealer.git
cd info-stealer
cargo build --release
```

## Utilisation

```bash
./target/release/rust
```
ou 

```bash
cargo run
```
## Dépendances

- `whoami` - Infos utilisateur
- `sysinfo` - Infos système
- `if-addrs` - Adresses réseau

## Licence

MIT