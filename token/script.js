function getUrlVars() {
    var vars = {};
    window.location.href.replace(/[#&]+([^=&]+)=([^&]*)/gi, (m,key,value) => {
        vars[key] = value;
    });
    return vars;
}

let token = getUrlVars()["access_token"];
let inputCopy = document.querySelector("#copy");
let inputToken = document.querySelector("#token");
let msg = document.querySelector(".token-message");

if (token == undefined) {
    msg.innerText = "Login failed";
} else {
    inputToken.value = token;
}

if (inputCopy != null) {
    inputCopy.addEventListener("click", () => {
        inputToken.select();
        inputToken.setSelectionRange(0, 99999);
        document.execCommand("copy");
    })
}
