import init,{main} from './addon-wasm/pkg/addon_wasm.js'
await init();
document.getElementById("output").textContent="output";
let isDistance=true;

function change_to_dist(){
	//cahges the thing we use to distance
	isDistance=true;
document.getElementById("fromval").style.display="inline";
document.getElementById("toval").style.display="inline";

document.getElementById("fromvalW").style.display="none";
document.getElementById("tovalW").style.display="none";
}


function change_to_weigth(){

	isDistance=false;
document.getElementById("fromval").style.display="none";
document.getElementById("toval").style.display="none";

document.getElementById("fromvalW").style.display="inline";
document.getElementById("tovalW").style.display="inline";
	
}



function calcul(){
//if is distance or something else
	if (isDistance){
		var from=document.getElementById("fromval").value;
		var to=document.getElementById("toval").value;
	}else{
		var from=document.getElementById("fromvalW").value;
		var to=document.getElementById("tovalW").value;
	}



	var num = document.getElementById("input").value;

	if (isNaN(num)){
	document.getElementById("output").textContent="input txt please";
	}else{
	document.getElementById("output").textContent=main(from,to,num).toFixed(2);
	}
}

document.getElementById("submitbutton").addEventListener("click",calcul);
document.getElementById("Distance").addEventListener("click",change_to_dist);
document.getElementById("Weigth").addEventListener("click",change_to_weigth);
