<script setup>
import Page from './components/Page.vue';
import { onMounted } from 'vue';

// document.getElementById('login-form').addEventListener('submit', function(event){
//     event.preventDefault();

//     let username = document.getElementById("username");
//     let password = document.getElementById("password");
//     const formData = new FormData(this);
//     console.log(formData);

//     // fetch('/login', {
//     //     method: 'POST',
//     //     body: JSON.stringify(formData.username, formData.password)
//     // })
//     // .then(response => {
//     //     if (!response.success)
//     //     {
//     //     throw new Error('Not ok');
//     //     }
//     // })
//     // .then(data => {
//     //     console.log(data);
//     // })
//     // .catch(error => {
//     //     console.error('Error: ', error);
//     // });
// });

onMounted( () =>
{
    let loginForm = document.getElementById("login-form");

    loginForm.addEventListener("submit", (e) => {
    e.preventDefault();

    let username = document.getElementById("username");
    let password = document.getElementById("password");

    if (username.value == "" || password.value == "") {
        alert("Ensure you input a value in both fields!");
    } else {
        // perform operation with form input
        alert("This form has been successfully submitted!");
        console.log(
        `This form has a username of ${username.value} and password of ${password.value}`
        );

        let formData = {
            username: username.value,
            password: password.value
        };

        fetch('/login', {
            method: 'POST',
            body: JSON.stringify(formData)
        })
        .then(response => {
            if (!response.success)
            {
                throw new Error('Not ok');
            }
        })
        .then(data => {
            console.log(data);
        })
        .catch(error => {
            console.error('Error: ', error);
        });

        username.value = "";
        password.value = "";
    }
    });}
)

</script>

<template>
    <Page>
        <form id="login-form" class="round-box">
            <label for="username">Username:</label>
            <input type="text" id="username" name="username" required>
            <label for="password">Password:</label>
            <input type="password" id="password" name="password" required>
            <button type="submit">Login</button>
        </form>
    </Page>
</template>

<style scoped>

.round-box {
    border-color: var(--color-primary);
    border-width: 0.2em;
    border-style: solid;
}

#login-form label {
    margin-bottom: 0.5em;
    font-weight: bold;
}

#login-form input {
    width: 100%;
    padding: 0.5em;
    margin-bottom: 1em;
    border: 1px solid var(--color-primary);
    border-radius: 4px;
    box-sizing: border-box;
    background-color: var(--color-background-mute);
}

#login-form button {
    padding: 0.5em 1em;
    background-color: darken(var(--color-primary), 10%);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

#login-form button:hover {
    background-color: var(--color-primary);
}
</style>