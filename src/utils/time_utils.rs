

pub fn print_time_spent( time: &std::time::Instant, prefix: &str )
{
    let mut elapse_millis: f64 = time.elapsed().as_millis() as f64;

    if elapse_millis > 2000.0 {
        println!(r#"{} {} s"#, prefix, format!("{:.3}", elapse_millis / 1000.0));
    } else {
        println!(r#"{} {} ms"#, prefix, format!("{:.1}", elapse_millis));
    }
}