var xhr = new XMLHttpRequest()
var host = window.location.hostname

function onLoad() {

}
function fn(){
	console.log("hi")
}
function getFileList(path) {
    xhr.open("GET",host+'/path/'+path)
}