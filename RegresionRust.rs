use std::time::Instant;

fn main() {
    // DATOS
    let x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    
    // PAÁMETROS
    let mut w: f64 = 0.0;      // pendiente
    let mut b: f64 = 0.0;      // intercepto
    let learning_rate: f64 = 0.01;
    let epochs: usize = 1000;
    let m: f64 = x.len() as f64;
    
    // ENTRENAMIENTO
    let inicio = Instant::now();
    
    for epoch in 0..epochs {
        // Calcular predicciones y errores
        let mut mse: f64 = 0.0;
        let mut dw: f64 = 0.0;
        let mut db: f64 = 0.0;
        
        // Iterar sobre cada ejemplo
        for i in 0..x.len() {
            let y_pred = w * x[i] + b;
            let error = y_pred - y[i];
            
            // Acumular gradientes
            dw += error * x[i];
            db += error;
            
            // Acumular error
            mse += error * error;
        }
        
        // Promediar los gradientes
        dw = (2.0 / m) * dw;
        db = (2.0 / m) * db;
        mse = mse / m;
        
        // Actualizar parámetros
        w -= learning_rate * dw;
        b -= learning_rate * db;
        
        // Mostrar progreso cada 200 
        if (epoch + 1) % 200 == 0 {
            println!("Epoch {}, MSE: {:.4}, w: {:.4}, b: {:.4}", 
                     epoch + 1, mse, w, b);
        }
    }
    
    let duracion = inicio.elapsed();
    
    // RESULTADOS
    println!("\nModelo entrenado:");
    println!("w ≈ {:.4}", w);
    println!("b ≈ {:.4}", b);
    
    // Predicción con nuevo valor
    let x_nuevo: f64 = 7.0;
    let y_pred_nuevo = w * x_nuevo + b;
    println!("Para x = {}, y_pred ≈ {:.4}", x_nuevo, y_pred_nuevo);
    
    // Mostrar tiempo
    println!("\nTiempo total de entrenamiento: {:.6} segundos", 
             duracion.as_secs_f64());
    println!("Tiempo en milisegundos: {:.2} ms", 
             duracion.as_secs_f64() * 1000.0);
}