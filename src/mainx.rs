fn main() {


    /* println!("Hello, world!"); */

    /*let kata = "world";
    let angka=5;

    println!("hello,{}!",kata);
    println!("balonku ada {}",angka);

    let meletus=1;
    let sisa=angka - meletus;
    println!("balonku tinggal {}",sisa); */

    /*let angka =5;
    let balon = angka;


    println!("balonku ada {}",angka);
    println!("balonku ada {}",balon);*/

    ///////materi aray1
    /* let numbers=vec![5,1,4]; //  array kumpulan dari angka
    println!("balonku ada {}",numbers[0]);//elemen ke 0
    println!("meletus {} balon",numbers[1]);//elemen ke 1
    println!("balon ku tinggal {}",numbers[2]);//elemen ke 2 
    */

    //////materi aray2
    /* let numbers=vec![5,1,4];
    let kumpulan = numbers;
    println!("balonku ada {}",kumpulan[0]);
    */

    // type i32
    let angka1 =5;
    let angka2: i32 = 7;

    println!("angka 1 & 2 : {angka1},{angka2}");

    // berbagai cara daam mendeklarasikan variable bertype string
    // cara berbeda beda , hasilnya sama : variable bertype string

    let warna1 ="hijau".to_string();
    let warna2=String::from("kuning");
    println!("warna 1 & 2 : {warna1},{warna2}");

    let warna2:String ="kelabu".into();
    let warna3:String ="merah muda".to_string();
    println!("warna 2 & 3 : {warna2},{warna3}");


}
