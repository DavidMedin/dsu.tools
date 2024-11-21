<script>
import Login from '@/Login.vue';

export default {
  methods: {
    toggleLogin() {
      // Logic to toggle the login form visibility
      const loginForm = document.getElementById('login-form');
      if (loginForm) {
        loginForm.style.display = loginForm.style.display === 'none' ? 'block' : 'none';
      }
    }
  }
};
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

#login-button,
#logout-button {
    padding-right: 1rem;
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
