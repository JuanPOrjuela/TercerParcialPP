import numpy as np
import time

# Datos
X = np.array([1, 2, 3, 4, 5], dtype=float)
y = np.array([2, 4, 6, 8, 10], dtype=float)

# Parametros
w = 0.0  # pendiente
b = 0.0  # intercepto
learning_rate = 0.01
epochs = 1000
m = len(X)  # número de ejemplos

# Entrenamiento mostrando el tiempo
tiempo_inicio = time.time()

for epoch in range(epochs):
    # Predicción del modelo: y_pred = w * X + b
    y_pred = w * X + b
    
    # Cálculo del error
    error = y_pred - y
    
    # Cálculo de gradientes (derivadas parciales)
    dw = (2/m) * np.dot(error, X)     # sum(error * X) * 2/m
    db = (2/m) * np.sum(error)        # sum(error) * 2/m
    
    # Actualización de parámetros
    w -= learning_rate * dw
    b -= learning_rate * db
    
    # (Opcional) Mostrar el costo cada cierto número de épocas
    if (epoch + 1) % 200 == 0:
        mse = np.mean(error ** 2)
        print(f"Epoch {epoch+1}, MSE: {mse:.4f}, w: {w:.4f}, b: {b:.4f}")

tiempo_final = time.time()
tiempo_total = tiempo_final - tiempo_inicio

# Resultados
print("\nModelo entrenado:")
print(f"w ≈ {w:.4f}, b ≈ {b:.4f}")

# Probar el modelo
x_nuevo = 7
y_pred_nuevo = w * x_nuevo + b
print(f"Para x = {x_nuevo}, y_pred ≈ {y_pred_nuevo:.4f}")

# Mostrar tiempo
print(f"\nTiempo total de entrenamiento: {tiempo_total:.6f} segundos")
print(f"Tiempo en milisegundos: {tiempo_total * 1000:.2f} ms")