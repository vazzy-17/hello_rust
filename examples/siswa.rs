#[derive(serde::Serialize)]

struct Siswa{
    nama:String,
    kelas:i8, // siswa smp: kelas 7,8,atau 9
    eksul:Option<String>
}

fn main() {
let siswa1 = Siswa{

    nama: "tono".into(),
    kelas:7,
    eksul:Some("Tenis Meja".into())

};
let siswa2 = Siswa{
    nama:"ucok".into(),
    kelas:8,
    eksul:None
};
println!("siswa1: {}/{}/{:?}",siswa1.nama,siswa1.kelas,siswa1.eksul);
println!("siswa2: {}/{}/{:?}",siswa2.nama,siswa2.kelas,siswa2.eksul);


let json_result = serde_json::to_string(&siswa1);
println!("json result:{:?}",json_result);
}