# 🦀 Jeu de devinette de mot – Rust

Un petit jeu en **Rust** inspiré du classique **Pendu / Word Guess**.  
Le joueur doit deviner un mot choisi au hasard, lettre par lettre, avant d'épuiser ses essais.

---

## 🎮 Règles du jeu

- Le programme choisit **un mot secret au hasard** parmi une liste prédéfinie :
  ```rust
  let mots = ["rust", "python", "ordinateur", "jeu", "programmation"];
Vous devez proposer des lettres une par une.

Si la lettre est dans le mot, elle s’affiche.
Sinon, vous perdez un essai.

Vous avez 6 essais maximum.

Le jeu se termine lorsque :

🎉 Vous avez trouvé toutes les lettres.

💀 Vous avez utilisé tous vos essais.
🦀 Technologies utilisées

Langage : Rust

Crates :

rand
 – pour le choix aléatoire
