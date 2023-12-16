# AES128

## About

En este repositorio encontraras la implementacion del block cipher <a href="https://es.wikipedia.org/wiki/Advanced_Encryption_Standard">AES (Advanced Encryption Standard)</a> que fue originalmente propuesta por el <a href="https://www.nist.gov/">NIST</a>. El documento formal del cual se ha basado esta implementacion es <a href="https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf">FIPS.197.pdf</a>. Tenga en cuenta que este codigo es simplemente para demostracion y no para otro uso. 

## Consideraciones

Este codigo no es recomendable para uso de encriptacion de documentos o texto de relevancia, ya que, entre muchas razones, no ofrece un mecanismo de padding y el modo de operacion no es el mas seguro como lo pudiese ser <a href="https://en.wikipedia.org/wiki/Galois/Counter_Mode">AES GCM</a>. A parte, como su nombre indica, se trata de la version de 128, la cual es la mas debil de las tres implementaciones posibles (128, 192 y 256).

## Estructura

A continuacion se describira el contenido de los diferentes archivos del repositorio:

- <ins>aes128.h</ins>: Declaracion de las dos funciones, tanto de encriptacion como de desencriptacion.
- <ins>aes128.c</ins>: Todas las funciones de cada etapa de las rondas.
- <ins>utils.c</ins>: Funciones para la generacion de las tabla SBox y su inversa.
- <ins>tests</ins>: Una carpeta con un test basado en el siguiente ejemplo: <a href="https://www.kavaliro.com/wp-content/uploads/2014/03/AES.pdf">AES.pdf</a>

## Test

Si quieres probar el test propuesto, ejecuta los siguientes comandos:

```c
cd tests
make test
```
