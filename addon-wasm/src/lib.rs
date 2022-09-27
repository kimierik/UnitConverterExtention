
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convertfromto()->f32{
    return 3.0 as f32
}

#[wasm_bindgen]
pub fn main(from:String,to:String,value:f32)->f32{
//turn value to meters first then turn it to whatever val we need
    let mut convert:f32=1.0;

    if from.eq(&String::from("m")){
        convert*=1.0;
    }
    if from.eq(&String::from("mile")){
        convert*=1609.34;
    }
    if from.eq(&String::from("inch")){
        convert*=0.0254;
    }
    if from.eq(&String::from("Km")){
        convert*=1000.0;
    }
    if from.eq(&String::from("feet")){
        convert*=0.3048;
    }


    if from.eq(&String::from("Kilo")){
        convert*=1.0;
    }
    if from.eq(&String::from("gram")){
        convert*=0.001;
    }

    if from.eq(&String::from("pound")){
        convert*=0.4536;
    }
    if from.eq(&String::from("ton")){
        convert*=907.18
    }
    if from.eq(&String::from("ounce")){
        convert*=0.02835;
    }





 

    if to.eq(&String::from("m")){
        convert/=1.0;
    }
    if to.eq(&String::from("mile")){
        convert/=1609.34;
    }
    if to.eq(&String::from("inch")){
        convert/=0.0254;
    }
    if to.eq(&String::from("Km")){
        convert/=1000.0;
    }
    if to.eq(&String::from("feet")){
        convert/=0.3048;
    }


    if to.eq(&String::from("Kilo")){
        convert/=1.0;
    }
    if to.eq(&String::from("gram")){
        convert/=0.001;
    }
    if to.eq(&String::from("pound")){
        convert/=0.4536;
    }
    if to.eq(&String::from("ton")){
        convert/=907.18
    }
    if to.eq(&String::from("ounce")){
        convert/=0.02835;
    }



convert*value
}

