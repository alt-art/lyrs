function getUrlVars() {
    var vars = {};
    window.location.href.replace(/[#&]+([^=&]+)=([^&]*)/gi, (m,key,value) => {
        vars[key] = value;
    });
    return vars;
}

let token = getUrlVars()["access_token"];


let inputToken = document.querySelector("#token");
if (token == undefined) {
    let divToken = document.querySelector(".token");
    divToken.innerHTML = "Login failed";
} else {
    inputToken.value = token;
}

let inputCopy = document.querySelector("#copy");

if (inputCopy != null) {
    inputCopy.addEventListener("click", ()=>{
        inputToken.select();
        inputToken.setSelectionRange(0, 99999);
        document.execCommand("copy");
    })
}
