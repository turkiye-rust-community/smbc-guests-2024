use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

/*
   Bu giriş demosunda basit bir sayı tahmin oyunu geliştiriyoruz.
   Program 1 ile 100 arasında bir sayı tutar ve oyuncunun bunu bilmesini ister.
*/

fn main() {
    // Oyuncuyu karşılama ritüeli
    println!("Sayı Tahmin Oyunu");
    println!("Aklımda 1 ile 100 arası bir sayı tuttum. Bakalım bilebilecek misin?");

    // rand crate kullanılarak 1 ile 100 arasında rastgele bir sayı tutulur
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // sonsuz döngü
    loop {
        // Oyuncu tahmini istenir
        println!("Aklındaki sayı nedir?");
        let mut guess_value = String::new();

        // terminalden girdi okunur, guess_value değişkenine alınır
        stdin()
            .read_line(&mut guess_value)
            .expect("Girdi okunamadı");

        // Sondaki \r\n karakteri temizlenir ve sayısal bir değer olup olmadığına bakılır
        let player_guess = match guess_value.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Tahminin {}", player_guess);

        // Pattern matching
        // Bilgisayarın tuttuğu sayı ile oyuncunun yazdığı değer kıyaslanır (compare)
        match player_guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Küçük bir sayı söyledin...")
            }
            Ordering::Equal => {
                println!("Bravo bildin...");
                break;
            }
            Ordering::Greater => {
                println!("Büyük bir sayı söyledin...")
            }
        }
    }
}
