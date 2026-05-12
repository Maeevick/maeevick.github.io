+++
title = "Trix Blasting"
description = "An almost simple shooting game inspired by my craziest dreams (or a space invasion)"
date = 2026-05-11
slug = "trix-blasting"
[extra]
locale = "en_US"
+++

A work in progress remake of Space Invaders to play with the last features of Bevy Engine 0.18.1: dodge, shoot and survive as long as possible — each hit, each miss, each new wave pushing the speed a little further.

## About the Game

Built with **Rust** and **Bevy Engine**, then transmuted into **WebAssembly** to play in the Ethereal Planes (and web browsers). _yeah it's the same formula than grimble-running!_

## Play Now

**Controls**: Way more complicated: 3 buttons instead of 1! 

1. Desktop
- **LEFT/RIGHT** or **A/D** or **Q/D**: move left and right
- **SPACE** or **CLICK**: shoot

2. Mobile
- **TAP + hold/slide below the baseline**: move left or right relative to Trix's position
- **TAP (release) anywhere**: shoot

> _"Hey my secret: spam below the baseline to blast continuously or aim and fire by releasing at the right moment!"_ ~ Trix 🧪💥

<div id="game-container" class="game-container">
  <button id="load-game-btn" class="cta">Play Trix Blasting</button>
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

## Developer's Notes

> _"Fire in the hole!"_ ~ Trix 🧪💥

Check out the [source code on GitHub](https://github.com/Maeevick/maeevick.github.io/tree/main/trix-blasting) to satisfy your curiosity or even propose ideas/improvements!
