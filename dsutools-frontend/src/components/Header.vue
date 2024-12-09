<script setup>
import { ref } from "vue";

let loggedIn = ref(localStorage.getItem("loggedIn") === "true");
let username = localStorage.getItem("username");

function logout() {
    const username = localStorage.getItem("username");

    if (loggedIn.value === null) {
        alert("You are not logged in!");
        return;
    }

    if (loggedIn.value && username != null) {
        fetch("/logout", {
            method: "POST",
            body: JSON.stringify({ username: username }),
            headers: {
                "Content-Type": "application/json",
            },
        }).then((response) => {
            if (!response.ok) {
                alert("Error logging out");
                throw new Error("Not ok");
            } else {
                // Ensure the elements exist before trying to access them
                if (loggedIn) {
                    localStorage.setItem("loggedIn", "false");
                    localStorage.removeItem("username");
                    window.location.replace("/");
                }
            }
        });
    }
}
</script>

<template>
    <header>
        <div class="name">
            <h1 class="dsu-blue" onclick="location.href = '/'">DSU.</h1>
            <h1 class="dsu-blue" onclick="location.href = '/'">TOOLS</h1>
        </div>
        <div v-if="loggedIn">
            <span>{{ username }}</span>
            <button id="logout-button" @click="logout()">Logout</button>
        </div>
        <template v-else>
            <div>
                <button onclick="location.href = '/register.html'">
                    Register
                </button>
                <button
                    id="login-button"
                    onclick="location.href = '/login.html'"
                >
                    Login
                </button>
            </div>
        </template>
    </header>
</template>

<style scoped>
.name {
    display: flex;
    padding: 1rem;
    align-items: center;
    user-select: none;
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
