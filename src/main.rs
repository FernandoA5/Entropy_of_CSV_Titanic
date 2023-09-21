#[derive(Debug)]
struct Dato {
    survived: i8,
    pclass: i8,
    sex: String,
    embarked: String,
}
fn main() {
    //Obtenemos los datos del CSV
    let datos = obtener_datos("detect.csv".to_string());
    
    //Creamos la primer variación de ordenamiento
    //Embarked, Pclass, Sex, Survived
    
    //Clasificamos los datos por embarked
    //Obtenemos los valores de embarked
    let embarked_values: Vec<String> = obtener_embarked_values(&datos);
    // println!("{:?}", embarked_values);
    //Obtenemos los valores de pclass
    let pclass_values: Vec<i8> = obtener_pclass_values(&datos);
    // println!("{:?}", pclass_values);
    //Obtenemos los valores de Sex
    let sex_values: Vec<String> = obtener_sex_values(&datos);
    // println!("{:?}", sex_values);
    //Obtenemos los valores de Survived
    let survived_values: Vec<i8> = obtener_survived_values(&datos);
    // println!("{:?}", survived_values);

    //Hecho eso, contamos los eventos de cada embarked_value
    let embarked_values_count: Vec<i32> = obtener_embarked_values_count(&datos, &embarked_values);
    println!("{:?}", embarked_values_count);

    //Calculamos las frecuencias de cada embarked_value
    let embarked_values_frecuency: Vec<f32> = obtener_embarked_values_frecuency(&datos, &embarked_values_count);
    println!("{:?}", embarked_values_frecuency);


}

fn obtener_embarked_values_frecuency(datos: &Vec<Dato>, embarked_values_count: &Vec<i32>) -> Vec<f32> {
    let mut embarked_values_frecuency: Vec<f32> = Vec::new();
    for value in embarked_values_count {
        embarked_values_frecuency.push(*value as f32 / datos.len() as f32);
    }
    embarked_values_frecuency
}
fn obtener_embarked_values_count(datos: &Vec<Dato>, values: &Vec<String>) -> Vec<i32> {
    let mut values_count: Vec<i32> = Vec::new();
    for value in values {
        let mut count = 0;
        for dato in datos {
            if dato.embarked == *value {
                count += 1;
            }
        }
        values_count.push(count);
    }
    values_count
}
fn obtener_survived_values(data: &Vec<Dato>) -> Vec<i8> {
    let mut survived_values: Vec<i8> = Vec::new();
    for value in data {
        if !survived_values.contains(&value.survived) {
            survived_values.push(value.survived);
        }
    }
    survived_values
}
fn obtener_sex_values(data: &Vec<Dato>) -> Vec<String> {
    let mut sex_values: Vec<String> = Vec::new();
    for value in data {
        if !sex_values.contains(&value.sex) {
            sex_values.push(value.sex.clone());
        }
    }
    sex_values
}
fn obtener_pclass_values(data: &Vec<Dato>) -> Vec<i8>{
    let mut pclass_values: Vec<i8> = Vec::new();
    for value in data{
        if !pclass_values.contains(&value.pclass){
            pclass_values.push(value.pclass);
        }
    }
    pclass_values
}
fn obtener_embarked_values(data: &Vec<Dato>)-> Vec<String>{
    let mut embarked_values: Vec<String> = Vec::new();
    for value in data{
        if !embarked_values.contains(&value.embarked){
            embarked_values.push(value.embarked.clone());
        }
    }
    embarked_values
}
fn obtener_datos(csv: String) -> Vec<Dato>{
    let mut datos: Vec<Dato> = Vec::new();
    let mut rdr = csv::Reader::from_path(csv).unwrap();
    
    for result in rdr.records() {
        let record = result.unwrap();
        let dato: Dato = Dato {
            survived: record[0].parse::<i8>().unwrap(),
            pclass: record[1].parse::<i8>().unwrap(),
            sex: record[2].to_string(),
            embarked: record[3].to_string(),
        };
        datos.push(dato);
    }
    datos
}