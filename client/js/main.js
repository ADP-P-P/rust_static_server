function onLoad() {

}
function fn(){
	console.log("hi")
}
function onFileClickListener(){
    
}
function getFileList(path) {
    var jqxhr=$.getJSON("/path?path="+path)
    .done(function(data){
        console.log(JSON.stringify(data,null,4))
        return data
    })
}