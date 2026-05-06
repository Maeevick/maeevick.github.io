+++
title = "Grimble Running"
description = "A simple platformer game inspired by our favorite library assistant (or a T-Rex)"
date = 2025-05-20
slug = "grimble-running"
[extra]
locale = "en_US"
+++

A ridiculously simple platformer game showcasing the extraordinary talent of our favorite goblin - _Grimble himself_ - for avoiding obstacles: mainly Trix's failed experiments and enchanted library books.

## About the Game

Built with **Rust** and **Bevy Engine**, then transmuted into **WebAssembly** to play in the Ethereal Planes (and web browsers).

## Play Now

**Controls**: It couldn't be simpler! **SPACE** or **CLICK/TOUCH**: Jump over obstacles

<div id="game-container" class="game-container">
  <button id="load-game-btn" class="cta">Play Grimble Running</button>
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

## Developer's Notes

> _"It's not magic, it's just sufficiently advanced engineering that looks like magic to people who don't read enough technical grimoires."_ ~ Trix 🧪💥

Check out the [source code on GitHub](https://github.com/Maeevick/maeevick.github.io/tree/main/grimble-running) to satisfy your curiosity or even propose ideas/improvements!
