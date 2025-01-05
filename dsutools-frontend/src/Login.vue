<script setup>
import Header from "./components/Header.vue";
import Page from "./components/Page.vue";
import { onMounted } from "vue";

onMounted(() => {
    let loginForm = document.getElementById("login-form");

    loginForm.addEventListener("submit", (e) => {
        e.preventDefault();

        if (localStorage.getItem("loggedIn") === "true") {
            alert("You are already logged in!");
            return;
        }

        let username = document.getElementById("username");
        let password = document.getElementById("password");

        if (username.value == "" || password.value == "") {
            alert("Ensure you input a value in both fields!");
            return;
        } else {
            // perform operation with form input
            // alert("This form has been successfully submitted!");
            console.log(
                `This form has a username of ${username.value} and password of ${password.value}`,
            );

            let username_verified = username.value;

            let formData = {
                username: username.value,
                password: password.value,
            };

            fetch("/login", {
                method: "POST",
                body: JSON.stringify(formData),
                headers: {
                    "Content-Type": "application/json",
                },
            })
                .then((response) => {
                    if (!response.ok) {
                        const message = document.createElement("p");
                        message.textContent = "Invalid username or password";
                        message.style.color = "red";
                        document.getElementById("login-form").appendChild(message);
                        
                        setTimeout(() => {
                            message.remove();
                        }, 5000);
                        // throw new Error("Not ok");
                    }
                    else{
                        localStorage.setItem("loggedIn", 'true');
                        localStorage.setItem("username", username_verified);
                        window.location.replace("/");
                    }
                })
                .catch((error) => {
                    console.error("Error: ", error);
                });

            document.getElementById("login-form").reset();
        }
    });
});
</script>

<template>
    <Page direction="column">
        <form id="login-form">
            <h1>Login</h1>
            <label for="username">Username:</label>
            <input type="text" id="username" name="username" required />
            <label for="password">Password:</label>
            <input type="password" id="password" name="password" required />
            <button type="submit">Login</button>
            <div style="display: flex; flex-direction: column; align-items: center;">
                <h2>Don't have an account?</h2>
                <button onclick="location.href = '/register.html'">Register</button>
            </div>
        </form>
    </Page>
</template>

<style scoped>
</style>
