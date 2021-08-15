function getUrlVars() {
    var vars = {};
    window.location.href.replace(/[#&]+([^=&]+)=([^&]*)/gi, (m,key,value) => {
        vars[key] = value;
    });
    return vars;
}

let token = getUrlVars()["access_token"];
let errorDescription = getUrlVars()["error_description"];
let inputCopy = document.querySelector("#copy");
let msg = document.querySelector(".token-message");
let errorDescriptionParagraph = document.querySelector("#error_description");

if (token == undefined) {
    msg.innerText = "Login failed";
    if (errorDescription != undefined) {
        errorDescriptionParagraph.innerText = errorDescription.split("+").join(" ");
    } else {
        window.location.replace("../");
    }
} else {
    msg.innerText = "Paste the token on terminal";
    inputCopy.style.display = "flex";
}

inputCopy.addEventListener("click", async () => {
    await navigator.clipboard.writeText(token);
})
