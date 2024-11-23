<script setup>
import { onMounted } from "vue";

// Call the function on page load to set initial state
onMounted(() => {
    console.log("Login button:", document.getElementById("login-button"));

    const loggedIn = localStorage.getItem('loggedIn') === 'true'; // Check stored value as string
  
    // Ensure the elements exist before trying to access them
    const loginButton = document.getElementById("login-button");
    const logoutButton = document.getElementById("logout-button");
  
    if (loginButton && logoutButton) {
      if (loggedIn) {
        loginButton.style.display = "none";
        logoutButton.style.display = "block";
      } else {
        loginButton.style.display = "block";
        logoutButton.style.display = "none";
      }
  }
},
);

</script>

<template>
    <header>
        <div class="name">
            <h1 class="dsu-blue" onclick="location.href = '/'">DSU.</h1>
            <h1 class="dsu-blue" onclick="location.href = '/'">TOOLS</h1>
        </div>
        <button id="login-button" onclick="location.href = '/login.html'">Login</button>
        <button id="logout-button" onclick="logout()" style="display: none;">Logout</button>
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
