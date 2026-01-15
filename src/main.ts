import { invoke } from '@tauri-apps/api/core';

const login_button = document.querySelector('#login_button');
const username_input = document.querySelector('input[placeholder="Username"]');
const password_input = document.querySelector('input[placeholder="Password"]');

login_button?.addEventListener("click", () => {
    login_validate();
});

async function login_validate() {
    const message = await invoke('login_validation', { name: username_input?.value, password: password_input?.value });
    console.log("login_validation called in typescript");

    if (message == true) {
        alert("Login Successful!");
    } else {
        alert("Login Failed. Please check your username and password.");
    }

}