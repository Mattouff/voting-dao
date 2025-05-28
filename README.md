# 🗳️ Solana Anchor Voting Program

Ce programme permet de créer, voter et supprimer des propositions de vote sur la blockchain Solana, en utilisant le framework Anchor.

## 🚀 Fonctionnalités

- Création de propositions avec titre, description, et entre 2 et 5 choix.
- Votes limités dans une période définie par des timestamps Unix.
- Suppression des propositions **par leur créateur uniquement** si elles sont closes depuis au moins 30 jours.

---

## 🛠️ Installation & Déploiement

### Prérequis

- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://book.anchor-lang.com/chapter_1/installation.html)
- Rust stable toolchain

### Clonage et compilation

```bash
git clone https://github.com/Mattouff/voting-dao
cd voting-dao
anchor build
```

## 📦 Structure du Programme

### `create_proposal`

Crée une nouvelle proposition de vote.

**Paramètres :**
- `title`: `String`
- `description`: `String`
- `choices`: `Vec<String>` (min 2, max 5)
- `date_start`: `u64` (timestamp Unix)
- `date_end`: `u64` (timestamp Unix)

**Erreurs possibles :**
- `InvalidNumberOfChoices`
- `DateNotConform`

---

### `cast_vote`

Vote pour un choix dans une proposition active.

**Paramètres :**
- `target`: `String` (nom du choix)

**Erreurs possibles :**
- `VoteNotOpen`
- `VoteClosed`
- `InvalidChoice`

---

### `delete_proposal`

Supprime une proposition **après 30 jours de sa clôture**.

**Conditions :**
- L'auteur du vote doit être le créateur
- Le vote doit être terminé depuis au moins 30 jours

**Erreurs possibles :**
- `NotAuthorized`
- `VoteNotEnded`
- `TooRecentToDelete`

---

## 🧾 Comptes Anchor

| Nom      | Type     | Description                                 |
|----------|----------|---------------------------------------------|
| Proposal | account  | Contient les métadonnées de la proposition  |
| Voting   | account  | Enregistre un vote individuel               |
| Choice   | struct   | Représente une option avec son nombre de votes |

---

## ⚠️ Codes d'erreurs

| Code                   | Description                                  |
|------------------------|----------------------------------------------|
| `DateNotConform`       | Date de début après la date de fin           |
| `InvalidNumberOfChoices` | Moins de 2 ou plus de 5 choix             |
| `InvalidChoice`        | Choix inexistant                             |
| `VoteNotOpen`          | Vote non encore ouvert                       |
| `VoteClosed`           | Vote déjà terminé                            |
| `NotAuthorized`        | Seul le créateur peut supprimer              |
| `VoteNotEnded`         | La proposition n'est pas encore finie        |
| `TooRecentToDelete`    | Moins de 30 jours depuis la fin              |

---

## 👤 Auteurs

Développé par **Mattouff**  
Déployé avec ❤️ sur Solana avec Anchor.

---

## 📄 Licence

MIT
