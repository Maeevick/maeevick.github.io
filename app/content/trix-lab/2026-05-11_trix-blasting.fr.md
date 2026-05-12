+++
title = "Trix Blasting"
description = "Un jeu de shoot presque simple inspiré de mes rêves les plus fous (ou d'une Invasion Spatiale)"
date = 2026-05-11
slug = "trix-blasting"
[extra]
locale = "fr_FR"
+++

Un remake de Space Invaders (work in progress) pour s'amuser avec les nouveautés de Bevy Engine 0.18.1 : esquive, tire et survis aussi longtemps que possible — chaque touche, chaque raté, chaque nouvelle vague pousse la vitesse un peu plus loin.

## À propos du jeu

Construit avec **Rust** et **Bevy Engine**, puis transmué en **WebAssembly** pour jouer dans les Plans Ethérés (et sur navigateurs web). _yeah c'est la même formule que pour grimble-running !_

## Jouer maintenant

**Contrôles** : Beaucoup plus compliqué: 3 boutons au lieu d'1 ! 

1. Desktop
- **GAUCHE/DROITE** or **A/D** or **Q/D**: déplacements vers la gauche ou la droite
- **ESPACE** or **CLICK**: shoot

2. Mobile
- **TAP + maintenir/glisser sous la ligne de base** : se déplacer à gauche ou à droite selon la position de Trix
- **TAP (relâché) n'importe où** : tirer

> _"Hej mon secret: spam sous la ligne pour blast en continue ou vise et tire en relachant au bon moment !"_ ~ Trix 🧪💥

<div id="game-container" class="game-container">
  <button id="load-game-btn" class="cta">Jouer à Trix Blasting</button>
  <div id="game-frame" style="display: none;">
    <iframe
      id="game-iframe" 
      style="width: 400px; height: 600px; border:1px solid black;"
      title="Trix Blasting"
      loading="lazy"
      allow="autoplay"
    ></iframe>
  </div>
</div>

<script>
document.getElementById('load-game-btn').addEventListener('click', function() {
  const gameIframe = document.getElementById('game-iframe');
  const gameFrame = document.getElementById('game-frame');
  const loadBtn = document.getElementById('load-game-btn');
  
  loadBtn.style.display = 'none';
  gameFrame.style.display = 'block';
  
  gameIframe.src = 'https://trix-blasting.s3.fr-par.scw.cloud/index.html';
});
</script>

<style>
.game-container {
  margin: 2rem auto;
  text-align: center;
  max-width: 400px;
}
</style>

## Notes du développeur

> _"Ça va péter !"_ ~ Trix 🧪💥

[Code source sur GitHub](https://github.com/Maeevick/maeevick.github.io/tree/main/trix-blasting) pour assouvir votre curiosité voire proposer des idées/améliorations !
