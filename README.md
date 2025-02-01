# Générateur de Fractales

Un générateur de fractales interactif écrit en Rust, utilisant egui pour l'interface graphique.

## Fonctionnalités

- **Types de Fractales Supportés**:
  - Ensemble de Mandelbrot
  - Ensemble de Julia
  - Burning Ship
  - Tricorn
  - Ensemble de Newton

- **Palettes de Couleurs**:
  - Classique
  - Feu
  - Océan
  - Arc-en-ciel
  - Niveaux de gris

- **Interactivité**:
  - Zoom avec la molette de la souris
  - Ajustement du nombre d'itérations
  - Sauvegarde d'images au format PNG

## Installation

1. Assurez-vous d'avoir Rust installé sur votre système
2. Clonez le repository :
# Générateur de Fractales

Un générateur de fractales interactif écrit en Rust, utilisant egui pour l'interface graphique.

## Fonctionnalités

- **Types de Fractales Supportés**:
  - Ensemble de Mandelbrot
  - Ensemble de Julia
  - Burning Ship
  - Tricorn
  - Ensemble de Newton

- **Palettes de Couleurs**:
  - Classique
  - Feu
  - Océan
  - Arc-en-ciel
  - Niveaux de gris

- **Interactivité**:
  - Zoom avec la molette de la souris
  - Ajustement du nombre d'itérations
  - Sauvegarde d'images au format PNG

## Installation

1. Assurez-vous d'avoir Rust installé sur votre système
2. Clonez le repository :

bash
git clone https://github.com/roketag33/FractalGenerator.git

3. Compilez et lancez le projet :

bash
cargo run --release


## Utilisation

- Utilisez la molette de la souris pour zoomer/dézoomer
- Sélectionnez différents types de fractales dans le menu "Type de fractale"
- Changez les couleurs via le menu "Palette de couleurs"
- Ajustez la précision avec le slider "Iterations"
- Sauvegardez vos créations avec le menu "Fichier > Sauvegarder l'image"

## Structure du Projet

Le projet suit les principes SOLID et DRY :
- `app.rs` : Gestion de l'état de l'application
- `fractal_calculator.rs` : Calcul des fractales
- `fractal_types.rs` : Définition des différents types de fractales
- `color_schemes.rs` : Gestion des palettes de couleurs
- `ui.rs` : Interface utilisateur
- `fractal_params.rs` : Paramètres des fractales

## Licence

MIT