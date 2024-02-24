use actix_web::{get, App, HttpServer, Responder};

fn find_primes(n: usize) -> Vec<usize> {
    let mut a: Vec<bool> = (0..=n).map(|i| i >= 2).collect();

    for i in 2..=(n as f64).sqrt() as usize {
        for j in 2..=(n / i) {
            a[i * j] = false;
        }
    }

    a.into_iter().enumerate().filter_map(|(i, x)| if x { Some(i) } else { None }).collect()
}


#[get("/primes")]
async fn prime_numbers() -> impl Responder {
    let primes = find_primes(100); // Adjust the range as needed
    format!("Prime numbers up to 100: {:?}\n", primes)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(prime_numbers)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}