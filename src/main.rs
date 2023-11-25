use std::io;

#[derive(Debug)]
struct Mahasiswa {
    nim: String,
    nama: String,
    jurusan: String,
}

fn tambah_mahasiswa(daftar_mahasiswa: &mut Vec<Mahasiswa>) {
    println!("Tambah Mahasiswa");

    println!("Masukkan NIM Mahasiswa:");
    let mut nim = String::new();
    io::stdin().read_line(&mut nim).expect("Gagal membaca nim");

    println!("Masukkan Nama Mahasiswa:");
    let mut nama = String::new();
    io::stdin()
        .read_line(&mut nama)
        .expect("Gagal membaca nama");

    println!("Masukkan Jurusan Mahasiswa:");
    let mut jurusan = String::new();
    io::stdin()
        .read_line(&mut jurusan)
        .expect("Gagal membaca jurusan");

    let mahasiswa_baru = Mahasiswa {
        nim: nim.trim().to_string(),
        nama: nama.trim().to_string(),
        jurusan: jurusan.trim().to_string(),
    };

    daftar_mahasiswa.push(mahasiswa_baru);
    println!("Mahasiswa berhasil ditambahkan!")
}

fn tampilkan_mahasiswa(daftar_mahasiswa: &Vec<Mahasiswa>) {
    println!("Data Mahasiswa");

    for mahasiswa in daftar_mahasiswa {
        println!("{:?}", mahasiswa);
    }
}

fn main() {
    let mut daftar_mahasiswa: Vec<Mahasiswa> = Vec::new();

    loop {
        println!("Menu:");
        println!("1. Tambah Mahasiwa");
        println!("2. Tampilkan Mahasiwa");
        println!("3. Keluar");

        let mut pilihan = String::new();
        io::stdin()
            .read_line(&mut pilihan)
            .expect("Pilihan tidak tersedia");

        match pilihan.trim().parse() {
            Ok(1) => tambah_mahasiswa(&mut daftar_mahasiswa),
            Ok(2) => tampilkan_mahasiswa(&daftar_mahasiswa),
            Ok(3) => {
                println!("Keluar dari program. selamat tinggal!");
                break;
            }
            _ => println!("Pilihan tidak valid. silahkan coba lagi."),
        }
    }
}
