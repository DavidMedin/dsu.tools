<script setup>
import Header from "./components/Header.vue";
import Page from "./components/Page.vue";
import { onMounted } from "vue";

onMounted(() => {
    let registrationForm = document.getElementById("register-form");

    registrationForm.addEventListener("submit", (e) => {
        e.preventDefault();

        let username = document.getElementById("username");
        let password = document.getElementById("password");

        if (username.value == "" || password.value == "") {
            alert("Ensure you input a value in both fields!");
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

            fetch("/register", {
                method: "POST",
                body: JSON.stringify(formData),
                headers: {
                    "Content-Type": "application/json",
                },
            })
                .then((response) => {
                    if (!response.ok) {
                        const message = document.createElement("p");
                        message.textContent = "The username is already taken";
                        message.style.color = "red";
                        document.getElementById("register-form").appendChild(message);
                        
                        setTimeout(() => {
                            message.remove();
                        }, 5000);
                        // throw new Error("Not ok");
                    }
                    else{
                        console.log(username_verified + " has been registered successfully!");
                        localStorage.setItem("loggedIn", 'true');
                        localStorage.setItem("username", username_verified);
                        window.location.replace("/");
                    }
                })
                .catch((error) => {
                    console.error("Error: ", error);
                });

            document.getElementById("register-form").reset();
        }
    });
});
</script>

<template>
    <Page>
        <form id="register-form" class="round-box">
            <h1>Register</h1>
            <label for="username">Username:</label>
            <input type="text" id="username" name="username" required />
            <label for="password">Password:</label>
            <input type="password" id="password" name="password" required />
            <button type="submit">Register</button>
        </form>
    </Page>
</template>

<style scoped>
.round-box {
    border-color: var(--color-primary);
    border-width: 0.2em;
    border-style: solid;
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 30%;
    height: 60%;
}

#register-form label {
    margin-bottom: 0.5em;
    font-weight: bold;
}

#register-form input {
    width: 100%;
    padding: 0.5em;
    margin-bottom: 1em;
    border: 1px solid var(--color-primary);
    border-radius: 4px;
    box-sizing: border-box;
    background-color: var(--color-background-mute);
}

#register-form button {
    padding: 0.5em 1em;
    background-color: darken(var(--color-primary), 10%);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

#register-form button:hover {
    background-color: var(--color-primary);
}
</style>
