fn main() {
    println!("cargo:rustc-env=DATABASE_URL=postgres://postgres:admin@127.0.0.1/postgres");
    println!("cargo:rustc-env=BASE_HREF=");
    println!("cargo:rustc-env=LISTEN_ADDRESS=127.0.0.1:8181");
    println!("cargo:rustc-env=HOST_TYPE=master");  // or slave
    println!("cargo:rustc-env=WEB_STYLE=solar");  // file.min.css
    println!("cargo:rustc-env=API_URL="); // set api url
}
