# TercerParcialPP

# Diseño de Soluciones y Comparación de Desempeño

---

## 1. Diseño de la Solución con Paradigma de Concurrencia y Cálculo PI

Este diseño toma la idea de calcular PI usando varios hilos. Cada hilo trabaja en una parte del problema y al final se suman todos los resultados para lograr la solución más rápido.

### Componentes que se implementan

**División de datos**
- Parte el conjunto de datos en bloques (uno por cada hilo disponible)
- Esto permite que cada hilo trabaje sólo con su sección y no todo el dataset

**Hilo de cálculo (trabajador)**
- Cada hilo calcula gradientes y error en su propio bloque de datos
- Es como calcular una parte de PI: cada quien lleva su parte y suma su propio resultado

**Sincronización**
- Cuando todos los hilos terminan, el programa los reúne
- Espera que terminen para sumar los resultados finales

**Suma final de resultados**
- Se suman los gradientes parciales de todos los hilos
- Se usan para actualizar los parámetros del modelo (`w` y `b`)

**Gestor de entrenamiento**
- Coordina el proceso general
- Divide los datos, crea los hilos, espera a que terminen y actualiza el modelo

### Arquitectura

**DatosParticion**
- Almacena cada bloque de datos que debe procesar un hilo

**HiloTrabajador**
- Recibe su bloque y calcula los gradientes allí
- Solo se preocupa por su sección

**GestorConcurrencia**
- Coordina la división, la creación de hilos, la sincronización y la suma final

### Beneficio principal

- El proceso de entrenamiento es mucho más rápido porque usa todos los núcleos del procesador de forma simultánea.
- El resultado total es la suma de todas las partes trabajadas a la vez.
- Se escala fácil si hay más datos o más núcleos, solo se aumentan hilos.

---

## 2. Diseño de la Solución con Paradigma de Aspectos

Este enfoque es diferente. La idea es separar lo importante (la regresión lineal) de las "cosas extras" (logging, validación, medición de tiempo, etc.). Así el código queda más limpio y ordenado.

### Aspectos que se implementan

**Aspecto de Validación**
- Verifica que los datos sean válidos antes de hacer cálculos
- Comprueba que X y y tengan el mismo tamaño
- Valida que el learning rate sea positivo
- Si algo está mal, detiene el proceso

**Aspecto de Logging**
- Registra lo que está pasando en el programa
- Anota cuándo comienza y termina el entrenamiento
- Guarda información de cada método ejecutado
- Es como un diario del programa

**Aspecto de Medición de Rendimiento**
- Mide cuánto tiempo tarda cada operación importante
- Al final, dice cuál fue la más lenta
- Ayuda a identificar cuellos de botella
- Genera reportes de desempeño

**Aspecto de Caché**
- Si ya calculamos algo antes y los parámetros no cambiaron, no lo vuelve a calcular
- Guarda el resultado anterior
- Lo reutiliza si es necesario
- Evita cálculos innecesarios

### Arquitectura

**RegresionLinealAspectos**
- Contiene solo la matemática pura
- Sin logging
- Sin validación
- Sin medición de tiempo
- Solo calcula

**GestorAspectos**
- Es un "envolvedor" que aplica automáticamente los aspectos
- Toma los métodos de la clase anterior
- Les añade validación, logging, etc.
- Todo ocurre de forma automática

### Beneficio principal

Si después queremos cambiar cómo funciona el logging, solo tenemos que cambiar el Aspecto de Logging. No tocamos la regresión lineal. El código es más fácil de mantener y entender.

---

## 3. Programa en Rust y Python - Comparación de Desempeño

### Para ejecutar los programas
- Debemos tener previamente instalado Python y Rust
- Ejecutamos los archivos desde la terminal usando `python RegresionPython.py`y para rust `rustc RegresionRust.rs` y `./RegresionRust.exe` para que nos muestre los resultados en la terminal como Python, ya que este genera el archivo en la carpeta, no muestra los resultados como python.
- Podemos seguir el ejemplo de la foto:
<img width="940" height="899" alt="image" src="https://github.com/user-attachments/assets/7f7651de-ef75-4681-9831-e5ee95e6707a" />


### Implementación

**Python**
- Usa NumPy para las operaciones
- Cada línea se interpreta cuando se ejecuta

**Rust**
- Usa vectores y ciclos simples
- Se compila antes de ejecutarse
- Convierte el código en instrucciones que el procesador entiende directamente

### Resultados de la Comparación

| Característica | Python | Rust |
|---|---|---|
| Tiempo de ejecución | 5.9 ms | 1.39 ms |
| Velocidad relativa | 1 | **4 veces más rápido** |
| Valor de `w` final | 1.9952 | 1.9952 |
| Valor de `b` final | 0.0174 | 0.0174 |
| Precisión | Igual | Igual |

### ¿Por qué Rust es mucho más rápido?

**Python interpreta línea por línea**
- Mientras se ejecuta, Python traduce cada instrucción
- Hay muchas pausas y decisiones en tiempo de ejecución

**Rust compila antes de ejecutar**
- Todo el código se traduce a instrucciones de máquina una sola vez
- Cuando lo ejecutamos, el procesador ya entiende todo perfectamente
- No hay traducción durante la ejecución

**Gestión de memoria**
- Python usa un recolector de basura que pausa la ejecución de vez en cuando
- Rust gestiona la memoria automáticamente sin pausas
- Rust no tiene estas interrupciones

**Optimizaciones**
- El compilador de Rust ve el código y lo reescribe de formas más eficientes
- Puede ejecutar múltiples operaciones juntas
- Evita cálculos innecesarios automáticamente

### Resultados matemáticos

Ambos lenguajes llegan exactamente a los mismos valores:
- `w = 1.9952`
- `b = 0.0174`

No hay diferencia en precisión o exactitud. La única diferencia es la velocidad de ejecución.

### Cuándo usar cada uno

**Python es mejor para:**
- Cuando queremos implementar un codigo mas facil
- Cuando escribir código es más importante que ejecutarlo rápido
- Cuando los datos son pequeños

**Rust es mejor para:**
- Cuando necesitamos mas velocidad
- Cuando tenemos millones de datos
- Cuando la velocidad importa mucho
