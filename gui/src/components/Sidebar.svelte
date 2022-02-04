<section class="sidebar">
  <img src="/images/fish1.png" alt="character" class="character-one" />
  <img alt="Fish Fight logo" src="images/logo.png" class="logo mt-4" />

  <div class="version-select">
    <label for="default_select">Versions</label>
    <div class="nes-select ">
      <select required id="default_select">
        <option value="0" selected>v0.4</option>
        <option value="1">v0.3</option>
      </select>
    </div>
  </div>

  <button type="button" class="nes-btn is-warning play-btn">Play!</button>

  <nav class="social">
    <a target="_blank" href="https://twitter.com/fishfightgame"
      ><i class="nes-icon twitter" /></a
    >
    <a target="_blank" href="https://github.com/fishfight">
      <i class="nes-icon github " /></a
    >
    <a target="_blank" href="https://discord.gg/rKmE4HTD">
      <i class="nes-icon discord" /></a
    >
  </nav>
</section>

<style lang="scss">
  .sidebar {
    padding: 0px 20px;
    display: flex;
    flex-direction: column;
    flex: 3;
    background-color: white;
    height: 100vh;
    position: relative;

    .logo {
      align-self: center;
      width: 70%;
      margin-top: 10px;
    }

    .version-select {
      margin-top: 20px;
    }

    .play-btn {
      margin-top: 20px;
      background-color: #ffc061;
    }

    .character-one {
      position: fixed;
      width: 200px;
      bottom: -100px;
      right: -50px;
    }

    .social {
      position: absolute;
      bottom: 10px;
      left: 60px;
    }
  }
</style>
