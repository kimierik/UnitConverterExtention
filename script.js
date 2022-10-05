import init,{App} from './addon-wasm/pkg/addon_wasm.js'
await init();
var app=new App();

document.getElementById("output").textContent="output";

app.set_isdist();


function change_to_dist(){
	app.set_isdist();
}


function change_to_weigth(){
	app.set_isweigth();
}



function calcul(){
	//if is distance or something else
	if (app.get_isdist()){
		var from=document.getElementById("fromval").value;
		var to=document.getElementById("toval").value;
	}else{
		var from=document.getElementById("fromvalW").value;
		var to=document.getElementById("tovalW").value;
	}
	//the above cannot be moved to wasm for now

	var num = document.getElementById("input").value;

	if (isNaN(num)){
	document.getElementById("output").textContent="Please input numbers";
	}else{
	document.getElementById("output").textContent=app.convert_from_table(from,to,num).toFixed(2);
	}
}

document.getElementById("submitbutton").addEventListener("click",calcul);
document.getElementById("Distance").addEventListener("click",change_to_dist);
document.getElementById("Weigth").addEventListener("click",change_to_weigth);
