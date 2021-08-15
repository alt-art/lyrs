let musics = [
    "Slipknot Duality",
    "AViVA GRRRLS",
    "Justin Bieber Justice",
    "Doja Cat Like That",
    "Linkin Park In The End",
];
let i = 0;
let musicTxt = document.querySelector('.music-example');

setInterval(() => {
    if (i > musics.length - 1)
        i = 0;

    musicTxt.innerText = musics[i];
    i++;
}, 3000);

let codeDownload = document.querySelector("#code-download");
let binaryDownload = document.querySelector("#binary-download");

fetch("https://api.github.com/repos/alt-art/lyrs/releases/latest").then((response) => {
    return response.json();
}).then((data) => {
    let zipUrl = data.zipball_url;
    codeDownload.href = zipUrl;
    let binUrl = data.assets[0].browser_download_url;
    binaryDownload.href = binUrl;
}).catch((reason) => {
    console.error("Error when requesting the last release", reason);
});