<script setup>

export function toggleLogin() {
  methods: {
    function toggleLogin() {
      // Logic to toggle the login form visibility
      if (localStorage.getItem('loggedIn') === true) {
        document.getElementById("login-button").style.display = "none";
        document.getElementById("logout-button").style.display = "block";
      }
      else if (localStorage.getItem('loggedIn') === false){
        document.getElementById("login-button").style.display = "block";
        document.getElementById("logout-button").style.display = "none";
      }
    }
  }
};
this.toggleLogin();
</script>

<template>
    <header>
        <div class="name">
            <h1 class="dsu-blue">DSU.</h1>
            <h1 class="dsu-blue">TOOLS</h1>
        </div>
        <button id="login-button" onclick="location.href = '/login.html'">Login</button>
        <button id="logout-button" onclick="location.href = '/logout.html'">Logout</button>
    </header>
</template>

<style scoped>
.name {
    display: flex;
    padding: 1rem;
    align-items: center;
    user-select: none;
}

#logout-button {
    display: none;
}

header {
    flex-grow: 0;
    display: flex;
    justify-content: space-between;
}

.dsu-blue {
    padding: 0;
    line-height: 1em;
}

header > * {
    align-self: center;
}
</style>
