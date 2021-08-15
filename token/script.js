function getUrlVars() {
    var vars = {};
    window.location.href.replace(/[#&]+([^=&]+)=([^&]*)/gi, (m,key,value) => {
        vars[key] = value;
    });
    return vars;
}

let token = getUrlVars()["access_token"];
let inputCopy = document.querySelector("#copy");
let msg = document.querySelector(".token-message");

if (token == undefined) {
    msg.innerText = "Login failed";
    inputCopy.style.display = "none"
}

inputCopy.addEventListener("click", async () => {
    await navigator.clipboard.writeText(token)
})
