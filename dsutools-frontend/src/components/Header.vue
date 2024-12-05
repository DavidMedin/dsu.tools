<script setup>
import { onMounted } from "vue";

// Call the function on page load to set initial state
onMounted(() => {
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

function logout() {
    const loggedIn = localStorage.getItem('loggedIn') === 'true'; // Check stored value as string
    const username = localStorage.getItem('username');

    if (loggedIn === null) {
        alert("You are not logged in!");
        return;
    }

    if (loggedIn && username != null) {
        fetch("/logout", {
            method: "POST",
            body: JSON.stringify({username: username}),
            headers: {
                "Content-Type": "application/json",
            },
        })
        .then((response) => {
            if (!response.ok) {
                alert("Error logging out");
                throw new Error("Not ok");
            }
            else{
                // Ensure the elements exist before trying to access them
                const loginButton = document.getElementById("login-button");
                const logoutButton = document.getElementById("logout-button");
              
                if (loginButton && logoutButton) {
                  if (loggedIn) {
                    loginButton.style.display = "block";
                    logoutButton.style.display = "none";
                    localStorage.setItem('loggedIn', 'false');
                    localStorage.removeItem('username');
                    window.location.replace('/');
                  } else {
                    loginButton.style.display = "none";
                    logoutButton.style.display = "block";
                  }
              }
            }
        })
    }
  
}

</script>

<template>
    <header>
        <div class="name">
            <h1 class="dsu-blue" onclick="location.href = '/'">DSU.</h1>
            <h1 class="dsu-blue" onclick="location.href = '/'">TOOLS</h1>
        </div>
        <button id="login-button" onclick="location.href = '/login.html'">Login</button>
        <button id="logout-button" @click="logout()" style="display: none;">Logout</button>
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
