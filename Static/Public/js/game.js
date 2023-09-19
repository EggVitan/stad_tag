let _Map, UserMarker, UserLatitude, UserLongitude, _Markers = [], _ConnectionDrawings=[]
let Websocket;
let DEBUGSocketData, DEBUGCooordinates, DEBUG
let playerIcon=""
let Players = []
let StepCount = 0, SteppedMarkers = []
function geolocationRun(position) {
    UserLatitude = position.coords.latitude;
    UserLongitude = position.coords.longitude;
    hideDices()
    initWS()
    MapInit();
    GetMapMarcers()
    _Map.invalidateSize();
    UpdateLocation();
    initWSListeners();
}
function initWS(){
    let groupName = document.getElementById("GroupName").value
    if(groupName == '')
    {
        return false
    }
    Websocket = new WebSocket("ws"+window.location.href.match(":\/.+?(?=\/)\/")+ //ws://(domain name)/WS
        "WS?Name="+groupName+"&PlayerIcon="+playerIcon+"&Lat="+UserLatitude+"&Lng="+UserLongitude
        ); 
    document.getElementById("BeforeWS").style.display="none"
    document.getElementById("map").style.display=""
    document.getElementById("DuringWS").style.display=""
    return true
}
function geolocationError(err) {
    console.log(err)
    alert("Unable to retrieve location: " + err);
}
function MapInit() {
    _Map = L.map("map").setView([UserLatitude, UserLongitude], 15); //15: zoom, this looks good i think
    L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").addTo(_Map);
    var useriCon = L.icon({
        iconUrl: '/static/user/icon/'+playerIcon,
        iconSize: [25, 25], // adjust size as needed
        iconAnchor: [5, 5], // half the icon size
        className: 'circular-marker' // specify custom CSS class
    });
    UserMarker = L.marker([UserLatitude, UserLongitude], {icon:useriCon}).addTo(_Map);
    //_Map.on('click', function(e){DrawClick(e)})
}
function UpdateLocation(){
    var prevLocation
    setInterval(function(){
        navigator.geolocation.getCurrentPosition(async function(position){
            try {
                if(position.coords.latitude == prevLocation.coords.latitude && position.coords.longitude == prevLocation.coords.longitude)
                    return
            }catch{
                prevLocation  = position
            }
            prevLocation  = position
            UserLatitude = position.coords.latitude;
            UserLongitude = position.coords.longitude;
            UserMarker.setLatLng([UserLatitude, UserLongitude])
            Websocket.send(JSON.stringify({Head:"UpdateLocation",la:UserLatitude, ln:UserLongitude}))
            CheckMarkerStatus()
        })
    }, 1000) //1s
}
function CheckMarkerStatus(){
    UserInMarker = NaN
    UserInMarkerIndex = NaN
    for(var i = 0; i < _Markers.length; i++){
        if(CalculateDistanceToMarker(_Markers[i]) < 10)
        {
            UserInMarkerIndex = i
            break
        }
    }
    if (isNaN(UserInMarkerIndex))
    {
        return
    }
    Websocket.send(JSON.stringify({
        Head:"InMarker",
        Marker: _Markers[UserInMarkerIndex].options.ID
    }))
    if(SteppedMarkersContains(_Markers[UserInMarkerIndex])){
        return
    } else {
        SteppedMarkers.push(_Markers[UserInMarkerIndex])
    }
    StepCount--
    shwSteps()
    if (StepCount != 0){
        return
    }
    Websocket.send(JSON.stringify({
        Head:"WalkedTheWay"
    }))
    StepCount = 0
    SteppedMarkers = []
    SteppedMarkers.push(_Markers[UserInMarkerIndex])

}
function SteppedMarkersContains(d) {
    for(let i = 0; i < SteppedMarkers.length; i++){
        if(d.options.ID == SteppedMarkers[i].options.ID){
            return true
        }
    }
    return false
}
function GetMapMarcers(){
    xhr = new XMLHttpRequest()
    xhr.open('POST', '')
    xhr.onload = SetMarcers
    xhr.send()
    
}
function SetMarcers(){
    console.log("HEJ!")
    console.log(this)
    data = JSON.parse(this.responseText)
    console.log(data)
    DEBUG = data
    data.sort(function(a,b){return a-b})
    for(var i = 0; i < data.length; i++){
        let markerData = data[i];
        let marker = L.circle(markerData.LatLng, {
            icon: L.icon({iconSize:[30,30]}),
            radius: 10
        })
        if (markerData.ContainsItem) {
            marker.options.color = 'green';
        }
        else if(markerData.IsExitLocation){
            marker.options.color = 'orange';
        }

        marker.options.ID = markerData.ID
        marker.options.LinksTo = markerData.ConnectsTo
        marker.options.ContainsItem = markerData.ContainsItem
        marker.options.IsExitLocation = markerData.IsExitLocation
        marker.options.Collected = false
        marker.bindPopup(
            `
                <input type='text' id='button_`+markerData.ID+`'>
                <button onClick='MarkerCollect(`+markerData.ID+`)'>Submit</button>
            `
        )
        CreatePolylines(marker)
        marker.addTo(_Map)
        _Markers.push(marker)
    }
    for(let i = 0; i < _ConnectionDrawings.length; i++){
        _ConnectionDrawings[i].bringToBack();
    }
}
function MarkerCollect(id){
    let marker =_Markers.find(e => e.options.ID===id)
    Websocket.send(JSON.stringify({
        Head:"FinishedControll",
        Key: document.getElementById("button_"+id).value,
        ID: id
    }))
}
function CreatePolylines(marker){
    for(let i = 0; i < marker.options.LinksTo.length; i++){
        let targetID = marker.options.LinksTo[i]
        if(marker.options.ID < targetID){ continue;}
        let target = _Markers.find(e => e.options.ID===targetID)
        var latlngs = [[
            target.getLatLng().lat,
            target.getLatLng().lng
        ], [
            marker.getLatLng().lat, 
            marker.getLatLng().lng
        ]];
        var polyline = L.polyline(latlngs, {weight: 5, color: 'red'})
        _ConnectionDrawings.push(polyline)
        polyline.addTo(_Map)

    }
}
function CalculateDistanceToMarker(marker){
    let userLatLng = L.latLng(UserLatitude, UserLongitude);
    let markerLatLng = marker.getLatLng();
    return userLatLng.distanceTo(markerLatLng);    
}

function rollDice(){
    if (StepCount > 0){return}
    dices = document.getElementsByClassName("dice");
    for(var i = 0; i < dices.length; i++){
        let d = dices[i]
        d.style.display = "none"
        
    }
    let rnd = Math.floor(Math.random() * 6);
    dices[rnd].style.display = ""
    StepCount = rnd+1 
    shwSteps()
}
function shwSteps(){
    document.getElementById("stegKvar").innerHTML = "<p>Gå " + StepCount + " steg</p>" 
}
function hideDices(){
    dices = document.getElementsByClassName("dice");
    for(var i = 1; i < dices.length; i++){
        let d = dices[i]
        d.style.display = "none"
        
    }

}

window.onload = function() {
    document.getElementById("sendIcon").addEventListener("click", function(event) {
        if (event.defaultPrevented) {
            // The default behavior of the event has already been prevented
            return;
        }
        event.preventDefault(); // Prevent the default form submission behavior

        const formData = new FormData(document.getElementById("AddImage")); // Get the form data
        fetch("/uploadPlayIcon", { method: "POST", body: formData }) // Send the form data using fetch()
            .then(response => response.text()) // Parse the response as text
            .then(data => {
                playerIcon = data; // Store the returned UUID in the global variable
                document.getElementById("AddImage").style.display="none"
                document.getElementById("startGameSendGroupName").style.display=""
                document.getElementById("startGame").addEventListener('click', startGame)
            })
            .catch(error => {
                console.error(error);
            });
        })
    };

function initWSListeners(){
    Websocket.addEventListener('open', function (event) {
    });
    
    Websocket.addEventListener('message', function (event) {
        let Data = JSON.parse(event.data)
        console.log(Data)
        switch(Data.Head)
        {
            case "Adduser":
                Adduser(Data)
                break
            case "UpdateUserLocation":
                UpdateUserLocation(Data)
                break
            case "RNDChallenge":
                RandomChallenge(Data)
        }
    });
}
function RandomChallenge(dat) {
    alert(dat.Challenge)
}
function Adduser(dat){
    var useriCon = L.icon({
        iconUrl: '/static/user/icon/'+dat.IconID,
        iconSize: [25, 25], // adjust size as needed
        iconAnchor: [5, 5], // half the icon size
        className: 'circular-marker' // specify custom CSS class
    });
    marker = L.marker([dat.Position.Lat, dat.Position.Lng], {icon: useriCon}).addTo(_Map)
    marker.options.ID = dat.Name
    Players.push(marker)
}
function UpdateUserLocation(dat){
    userFound = false
    Players.forEach(mark => {
        if(mark.options.ID == dat.Name){
            mark.setLatLng({lat:dat.Position.Lat,lng:dat.Position.Lng})
            userFound = true
        }
    });
    if (!userFound){
        Websocket.send(JSON.stringify({Head:"AddUser",Name:dat.Name}))
    }
}
function startGame(){
    if (navigator.geolocation) {
        navigator.geolocation.getCurrentPosition(geolocationRun, geolocationError);
    } else {
        alert("Geolocation is not supported by this browser");
    }
}