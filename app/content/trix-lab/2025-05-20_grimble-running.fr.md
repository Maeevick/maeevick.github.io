+++
title = "Grimble Running"
description = "Un jeu de plateforme simple inspiré de notre assistant de bibliothèque préféré (ou d'un T-Rex)"
date = 2025-05-20
slug = "grimble-running"
[extra]
locale = "fr_FR"
+++

Un jeu de plateforme stupidement simple basé sur le talent extraordinaire de notre gobelin préféré - _j'ai nommé Grimble_ - pour éviter les obstacles : principalement les expériences ratées de Trix et les livres envoûtés de la bibliothèque.

## À propos du jeu

Construit avec **Rust** et **Bevy Engine**, puis transmué en **WebAssembly** pour jouer dans les Plans Ethérés (et sur navigateurs web).

## Jouer maintenant

**Contrôles** : Il n'y a pas plus simple ! **ESPACE** ou **CLICK/TOUCH** : Sauter par-dessus les obstacles

<div id="game-container" class="game-container">
  <button id="load-game-btn" class="cta">Jouer à Grimble Running</button>
  <div id="game-frame" style="display: none;">
    <iframe
      id="game-iframe" 
      style="width: 600px; height: 200px; border:1px solid black;"
      title="Grimble Running"
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
  
  gameIframe.src = 'https://grimble-running.s3.fr-par.scw.cloud/index.html';
});
</script>

<style>
.game-container {
  margin: 2rem auto;
  text-align: center;
  max-width: 600px;
}
</style>

## Notes du développeur

> _"Ce n'est pas de la magie, c'est juste de l'ingénierie suffisamment avancée qui ressemble à de la magie pour les personnes qui ne lisent pas assez de grimoires techniques."_ ~ Trix 🧪💥

[Code source sur GitHub](https://github.com/Maeevick/maeevick.github.io/tree/main/grimble-running) pour assouvir votre curiosité voire proposer des idées/améliorations !
