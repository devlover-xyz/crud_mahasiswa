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

fn edit_mahasiswa(daftar_mahasiswa: &mut Vec<Mahasiswa>) {
    println!("Masukkan NIM mahasiswa yang akan diedit:");

    let mut nim_edit = String::new();
    io::stdin()
        .read_line(&mut nim_edit)
        .expect("Gagal membaca NIM");

    if let Some(mahasiswa) = daftar_mahasiswa
        .iter_mut()
        .find(|m| m.nim == nim_edit.trim())
    {
        println!("Masukkan NIM mahasiswa baru:");
        let mut nim_baru = String::new();
        io::stdin()
            .read_line(&mut nim_baru)
            .expect("Gagal membaca NIM");

        println!("Masukkan nama mahasiswa baru:");
        let mut nama_baru = String::new();
        io::stdin()
            .read_line(&mut nama_baru)
            .expect("Gagal membaca nama");

        println!("Masukkan jurusan mahasiswa baru:");
        let mut jurusan_baru = String::new();
        io::stdin()
            .read_line(&mut jurusan_baru)
            .expect("Gagal membaca jurusan");

        mahasiswa.nim = nim_baru.trim().to_string();
        mahasiswa.nama = nama_baru.trim().to_string();
        mahasiswa.jurusan = jurusan_baru.trim().to_string();

        println!("Mahasiswa berhasil diubah!");
    } else {
        println!("Mahasiswa dengan NIM {} tidak ditemukan.", nim_edit.trim());
    }
}

fn hapus_mahasiswa(daftar_mahasiswa: &mut Vec<Mahasiswa>) {
    println!("Masukkan NIM mahasiswa yang akan dihapus:");

    let mut nim_hapus = String::new();
    io::stdin()
        .read_line(&mut nim_hapus)
        .expect("Gagal membaca NIM");

    if let Some(index) = daftar_mahasiswa.iter().position(|m|m.nim == nim_hapus.trim())  {
        daftar_mahasiswa.remove(index);
        println!("Mahasiswa berhasil dihapus!");
    }else{
        println!("Mahasiswa dengan NIM {} tidak ditemukan.", nim_hapus.trim());
    }
}

fn main() {
    let mut daftar_mahasiswa: Vec<Mahasiswa> = Vec::new();

    loop {
        println!("Menu:");
        println!("1. Tambah Mahasiwa");
        println!("2. Tampilkan Mahasiwa");
        println!("3. Edit Mahasiwa");
        println!("4. Hapus Mahasiwa");
        println!("9. Keluar");

        let mut pilihan = String::new();
        io::stdin()
            .read_line(&mut pilihan)
            .expect("Pilihan tidak tersedia");

        match pilihan.trim().parse() {
            Ok(1) => tambah_mahasiswa(&mut daftar_mahasiswa),
            Ok(2) => tampilkan_mahasiswa(&daftar_mahasiswa),
            Ok(3) => edit_mahasiswa(&mut daftar_mahasiswa),
            Ok(4) => hapus_mahasiswa(&mut daftar_mahasiswa),
            Ok(9) => {
                println!("Keluar dari program. selamat tinggal!");
                break;
            }
            _ => println!("Pilihan tidak valid. silahkan coba lagi."),
        }
    }
}
