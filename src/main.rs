use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

use rand::Rng;

slint::include_modules!();

struct UserInput{
    type_index:i32,
    range_index:i32,
    cnt_index:i32,
}

pub fn main() {



    let app = App::new().unwrap();
    {
        let app_weak = app.as_weak();

        app.on_genBtnClicked(move || {
            let app = app_weak.unwrap();
            let user_input = UserInput{
                type_index:app.get_typeIndex(),
                range_index:app.get_rangeIndex(),
                cnt_index:app.get_cntIndex(),
            };
            gen_doc(user_input);

        });
    }
   
   
    app.run().unwrap();
}


fn gen_doc(user_input:UserInput){


 
    let  operator_symbol =  match user_input.type_index {
        0=>{
            '+'
        },
        1=>{
           '-'
        },
        2=>{
            '×'
        },
        3=>{
             '÷'
        },   
        _=>{
            '+'
        }
    };
    let max_num = match user_input.range_index {
        0=>{
           10
        },
        1=>{
          20
        },
        2=>{
           50
        },
        3=>{
            100
        }
        _=>{
            10}
    };

    let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_external_font(File::open("assets/fonts/RobotoMedium.ttf").unwrap()).unwrap();
    let mut rng = rand::thread_rng();
    let x_item1 = 15.0;
    let x_item2 = 85.0;
    let x_item3 = 155.0;

    for row in 0..=24 {

        for col in 0..=2{
            let x_pos = match col{
                0=>{
                    x_item1
                },
                1=>{
                    x_item2
                },
                2=>{
                    x_item3
                },  
                _=>{
                    x_item1
                }
            };
            let mut operat_num1 = rng.gen_range(1..max_num);
            let mut operat_num2 = rng.gen_range(1..max_num);
            if  operator_symbol == '-' || operator_symbol == '÷' {
                operat_num1 = rng.gen_range(2..max_num);
                operat_num2 = rng.gen_range(1..operat_num1);

            }
    

            let cur_text = match user_input.cnt_index {
                0=>{
                    format!("{item1} {operator_symbol} {item2} =   ",
                    item1 = operat_num1,
                    operator_symbol = operator_symbol,
                    item2 = operat_num2)
                },
                1=>{
                    let mut operat_num3 = rng.gen_range(1..max_num);
                    if  operator_symbol == '-' || operator_symbol == '÷' {
                        operat_num1 = rng.gen_range(3..max_num);
                        operat_num2 = rng.gen_range(2..operat_num1);
                        operat_num3 = rng.gen_range(1..operat_num2);

                    }
                    format!("{item1} {operator_symbol} {item2} {operator_symbol} {item3} =   ",
                    item1 = operat_num1,
                    operator_symbol = operator_symbol,
                    item2 = operat_num2,
                    item3 = operat_num3)
                },
                2=>{
                    let mut operat_num3 = rng.gen_range(1..max_num);
                    let mut operat_num4 = rng.gen_range(1..max_num);
                    if  operator_symbol == '-' || operator_symbol == '÷' {
                        operat_num1 = rng.gen_range(4..max_num);
                        operat_num2 = rng.gen_range(3..operat_num1);
                        operat_num3 = rng.gen_range(2..operat_num2);
                        operat_num4 = rng.gen_range(1..operat_num3);

                    }
                    format!("{item1} {operator_symbol} {item2} {operator_symbol} {item3} {operator_symbol} {item4}=   ",
                    item1 = operat_num1,
                    operator_symbol = operator_symbol,
                    item2 = operat_num2,
                    item3 = operat_num3,
                    item4 = operat_num4)
                },
                _=>{
                    format!("{item1} {operator_symbol} {item2} =   ",
                    item1 = operat_num1,
                    operator_symbol = operator_symbol,
                    item2 = operat_num2)
                }
            };
            current_layer.use_text(cur_text,  15.0, Mm(x_pos), Mm(270.0-row as f32 *(270.0/25.0)), &font);
        }

     
    }


    doc.save(&mut BufWriter::new(File::create("test_working.pdf").unwrap())).unwrap();
}