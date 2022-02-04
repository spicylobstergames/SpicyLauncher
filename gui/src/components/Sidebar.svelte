<section class="sidebar">
  <img alt="Fish Fight logo" src="images/logo.png" class="logo mt-4" />

  <div class="version-select">
    <label for="default_select">Versions</label>
    <div class="nes-select ">
      <select required id="default_select">
        <option value="" disabled selected hidden>Select...</option>
        <option value="0">To be</option>
        <option value="1">Not to be</option>
      </select>
    </div>
  </div>

  <img src="/images/fish1.png" alt="character" class="character-one" />
</section>

<style lang="scss">
  .sidebar {
    display: flex;
    flex-direction: column;
    flex: 3;
    background-color: white;
    height: 100vh;

    .logo {
      align-self: center;
      width: 70%;
      margin-top: 10px;
    }

    .version-select {
      margin-top: 20px;
      padding: 0 10px;
    }
    .character-one {
      position: fixed;
      width: 200px;
      bottom: -100px;
      right: -50px;
    }
  }
</style>
