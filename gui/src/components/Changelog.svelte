<div class="changelog">
  <img src="/images/fish2.png" alt="character" class="character-two" />
  <img src="/images/fish3.png" alt="character" class="character-three" />
</div>

<style lang="scss">
  .changelog {
    flex: 5;
    position: relative;

    .character-two {
      position: absolute;
      right: -130px;
      z-index: -10;
    }

    .character-three {
      position: fixed;
      bottom: -20px;
      left: 0px;
    }
  }
</style>
