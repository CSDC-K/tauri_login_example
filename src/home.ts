import { invoke } from '@tauri-apps/api/core';


const urlParams = new URLSearchParams(window.location.search);
const username = urlParams.get('username');

if (username) {
    const LOGIN_usertitle = document.querySelector("#home-title") as HTMLHeadingElement;
    LOGIN_usertitle.textContent = `${username}`;
}