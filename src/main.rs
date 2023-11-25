use std::io;

fn main() {
    
    loop {
        println!("Menu:");
        println!("1. Tambah Mahasiwa");
        println!("2. Tampilkan Mahasiwa");
        println!("3. Keluar");

        let mut pilihan = String::new();
        io::stdin().read_line(&mut pilihan).expect("Pilihan tidak tersedia");

        match pilihan.trim().parse() {
            Ok(1) => println!("Menu Tambah mahasiswa"),
            Ok(2) => println!("Menu Tampilkan mahasiswa"),
            Ok(3) => {
                println!("Keluar dari program. selamat tinggal!");
                break;
            },
            _ => println!("Pilihan tidak valid. silahkan coba lagi.")
        }
    }
}