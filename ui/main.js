document.addEventListener("keydown", (event) => {
    const cmdKey = event.metaKey || event.ctrlKey;
    if (cmdKey && event.key === "r") {
        event.preventDefault();
        window.location.reload();
    }
});



//const socket = new WebSocket("ws://localhost:8080/ws2/");
const socket2 = new WebSocket("ws://localhost:8080/ws/");

socket2.addEventListener("open", (event) => {
    console.log("Second socket has being opened", event);
});

socket2.addEventListener("message", (event) => {
    console.log("WebSocket message received:", event.data);
    if(event.data === "reload"){
        console.warn("RELOAD!!!!")
        window.location.reload() ;
    }
    document.getElementById("showSocketMessages").textContent = event.data
})



//socket.addEventListener("open", (event) => {
//    console.log("WebSocket connection opened:", event);
//});
//
//socket.addEventListener("message", (event) => {
//    console.log("WebSocket message received:", event.data);
//});

function sendMessage(input) {
      socket2.send(input.value);
}



window.addEventListener("load", init)

function init(){
    const socketInput = document.getElementById("socketInput");
    const ourButton = document.getElementById("openSocket");
    ourButton.addEventListener("click", ()=>sendMessage(socketInput))

    const picture_form = document.getElementById("postPicture");

    picture_form.addEventListener("submit", formHandler)
}


function formHandler(event){
        event.preventDefault()
        console.log("Submitting form :O",event.target.files);
}

