use std::io;
use rand::prelude::IndexedRandom;

fn main() {
    println!("=== Jeu de devinette de mot ===");

    let mots = ["rust", "python", "ordinateur", "jeu", "programmation"];
    let mot_secret = mots.choose(&mut rand::rng()).unwrap();

    let mut masque: Vec<char> = mot_secret.chars().map(|_| '_').collect();
    let mut essais = 6;
    let mut lettres_trouvees: Vec<char> = Vec::new();

    println!("Le mot à deviner a {} lettres.", mot_secret.len());

    loop {
        println!("\nMot actuel : {}", masque.iter().collect::<String>());
        println!("Il vous reste {} essais.", essais);
        println!("Entrez une lettre :");

        let mut entree = String::new();
        io::stdin().read_line(&mut entree).expect("Erreur de saisie");
        let entree = entree.trim().to_lowercase();

        if entree.len() != 1 {
            println!("Veuillez entrer une seule lettre.");
            continue;
        }

        let lettre = entree.chars().next().unwrap();

        if lettres_trouvees.contains(&lettre) {
            println!("Vous avez déjà proposé '{}'.", lettre);
            continue;
        }
        lettres_trouvees.push(lettre);

        // Ici, on crée une "liste de choix" selon que la lettre est présente ou non
        let resultat = if mot_secret.contains(lettre) {
            ["trouve"]
        } else {
            ["rate"]
        };

        match resultat.choose(&mut rand::rng()) {
            Some(&"trouve") => {
                println!("Bien joué ! La lettre '{}' est dans le mot.", lettre);
                for (i, c) in mot_secret.chars().enumerate() {
                    if c == lettre {
                        masque[i] = lettre;
                    }
                }
            }
            Some(&"rate") => {
                println!("Raté ! La lettre '{}' n'est pas dans le mot.", lettre);
                essais -= 1;
            }
            _ => {}
        }

        if !masque.contains(&'_') {
            println!("\n🎉 Bravo ! Vous avez deviné le mot : {}", mot_secret);
            break;
        }

        if essais == 0 {
            println!("\n💀 Vous avez perdu ! Le mot était : {}", mot_secret);
            break;
        }
    }
}
