use std::collections::HashMap;

use wasm_bindgen::prelude::*;





#[wasm_bindgen]
pub struct App{
    is_distance:bool,
    conversion_table:HashMap<String,f32>,
    window:web_sys::Window,
}

#[wasm_bindgen]
impl App{
    #[wasm_bindgen(constructor)]
    pub fn new()->Self{
        App{
        is_distance:true,
        window:web_sys::window().expect("nowindow"),
        conversion_table:HashMap::from([
            //x*f32 = meter
            ("meter".to_string(),1.0),
            ("mile".to_string(),1609.34),
            ("inch".to_string(),0.0254),
            ("Kilometer".to_string(),1000.0),
            ("feet".to_string(),0.3048),

            //x*f32=kg
            ("Kilogram".to_string(),1.0),
            ("gram".to_string(),0.001),
            ("pound".to_string(),0.4536),
            ("ton".to_string(),907.18),
            ("ounce".to_string(),0.02835),
        ])
        }
    }


    pub fn changetxt(&self){
        let document=self.window.document().expect("no document from windo");
        let val= document.get_element_by_id("title");
        val.unwrap().set_class_name("hide");
    }




    pub fn set_isdist(&mut self){
        let document=self.window.document().expect("no document from windo");
        self.is_distance=true;
        document.get_element_by_id("fromval").unwrap().set_class_name("show");
        document.get_element_by_id("toval").unwrap().set_class_name("show");

        document.get_element_by_id("fromvalW").unwrap().set_class_name("hide");
        document.get_element_by_id("tovalW").unwrap().set_class_name("hide");


    }
    pub fn set_isweigth(&mut self){
        let document=self.window.document().expect("no document from windo");
        self.is_distance=false;
        document.get_element_by_id("fromval").unwrap().set_class_name("hide");
        document.get_element_by_id("toval").unwrap().set_class_name("hide");

        document.get_element_by_id("fromvalW").unwrap().set_class_name("show");
        document.get_element_by_id("tovalW").unwrap().set_class_name("show");
    }
    pub fn get_isdist(&self)->bool{
        self.is_distance
    }

    pub fn convert_from_table(&self,from:String,to:String,value:f32)->f32{
        let a=self.conversion_table.get(&from);
        let b=self.conversion_table.get(&to);
        return value*a.unwrap()/b.unwrap();
    }

    /*
    pub fn convert_fn(&self,from:String,to:String,value:f32)->f32{
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
*/
}





